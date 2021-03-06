use {
    crate::{Config, LoopCounter, Package},
    crate::util::writer,
    byteorder::{BigEndian, ReadBytesExt, WriteBytesExt},
    serde_json::{self, Value},
    flate2::bufread::ZlibDecoder,
    std::{
        io::{self, Cursor, Read, Write},
        net::{TcpStream, ToSocketAddrs},
        str,
        sync::mpsc::{self, TryRecvError},
        thread,
        time::Duration,
    },
};

pub fn main_loop<B: Write>(config: Config, buf: &mut B) -> io::Result<()> {
    loop {
        let socket = match format!("{}:{}", config.host, config.port).to_socket_addrs() {
            Ok(addrs) => {
                let mut socket = None;
                for addr in addrs {
                    println!("正在连接服务器...");
                    match TcpStream::connect_timeout(&addr, Duration::from_secs(20)) {
                        Ok(s) => {
                            socket = Some(s);
                            break;
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
                socket
            }
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        };

        let mut stream: TcpStream = match socket {
            None => {
                // If connection failed, retry after 3 secs.
                eprintln!("无法连接服务器，3秒后尝试重连...");
                thread::sleep(Duration::from_secs(3));
                continue;
            }
            Some(s) => s,
        };

        println!("成功连接服务器...");
        println!("请求进入直播间...");
        if let Err(e) = send(
            &mut stream,
            Package::join_room(config.user_id, config.room_id),
        ) {
            // Retry if connection failed.
            println!("进入直播间失败: {}", e);
            continue;
        };

        // Spawn a thread for continuously sending heartbeat package in the background.
        let (tx, rx) = mpsc::channel();
        let mut heartbeat_stream = match stream.try_clone() {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("stream clone failed: {}", e);
                thread::sleep(Duration::from_secs(3));
                continue;
            }
        };
        thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("terminating heartbeat thread");
                    break;
                }
                Err(TryRecvError::Empty) => (),
            }

            // Sleeping 30s before sending another heartbeat package if succeeded.
            // Otherwise retrying after 10s.
            if let Err(e) = send(&mut heartbeat_stream, Package::heartbeat()) {
                eprintln!("sending heartbeat package failed: {}", e);
                thread::sleep(Duration::from_secs(10));
            } else {
                thread::sleep(Duration::from_secs(30));
            }
        });

        if let Err(e) = receive(&mut stream, buf, &config) {
            eprintln!("network failure: {}", e);
        }

        if let Err(e) = tx.send(()) {
            eprintln!("error terminating child thread: {}", e);
        }
    }
}

fn send<T: Write>(stream: &mut T, package: Package) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::with_capacity(package.length);
    buffer.write_u32::<BigEndian>(package.length as u32)?;
    buffer.write_u32::<BigEndian>(package.version as u32)?;
    buffer.write_u32::<BigEndian>(package.opcode as u32)?;
    buffer.write_u32::<BigEndian>(package.param as u32)?;
    match package.body {
        Some(body) => buffer.extend_from_slice(body.as_bytes()),
        None => (),
    };
    stream.write_all(buffer.as_slice())?;
    Ok(())
}

fn receive<T: Read, B: Write>(
    socket: &mut T,
    buf: &mut B,
    config: &Config,
) -> Result<(), &'static str> {
    let mut counter = LoopCounter::new(config.log_interval as f64);

    loop {
        // Parse the header
        let mut header = [0u8; 16];
        socket
            .read_exact(&mut header)
            .or(Err("error reading socket"))?;
        let mut cur = Cursor::new(header);
        let mut package = Package {
            length: cur
                .read_u32::<BigEndian>()
                .or(Err("error parsing length"))? as usize,
            version: cur
                .read_u32::<BigEndian>()
                .or(Err("error parsing version"))?,
            opcode: cur
                .read_u32::<BigEndian>()
                .or(Err("error parsing action"))?,
            param: cur.read_u32::<BigEndian>().or(Err("error parsing param"))?,
            body: None,
        };

        // Parse the body
        let mut buffer = vec![0u8; package.length - 16];
        if package.length > 16 {
            socket
                .read_exact(buffer.as_mut_slice())
                .or(Err("error reading socket"))?;
            package.body = Some(
                //str::from_utf8(buffer.as_slice())
                //    .unwrap_or("error decoding utf8")
                //    .to_owned(),
                decode_bufreader(buffer.as_slice()).unwrap_or("error decoding package body".to_owned())
            );
        }

        match package.opcode {
            3 => {
                if counter.next().unwrap() == config.log_interval as f64 {
                    let popularity = Cursor::new(buffer.as_slice())
                        .read_u32::<BigEndian>()
                        .unwrap();
                    writer::write_popularity(popularity, buf, config.log_threshold);
                }
            }
            8 => println!("成功进入直播间, 开始监听..."),
            5 => {
                let data = String::from_utf8(buffer).unwrap_or("error decoding utf8".to_string());
                if let Err(e) = parse_barrage(&data, buf, &config) {
                    eprintln!("error parsing danmu: {}", e);
                }
            }
            _ => {
                eprintln!("unknown package: {:#?}", package);
            }
        }
    }
}

fn parse_barrage<B: Write>(data: &str, buf: &mut B, config: &Config) -> Result<(), &'static str> {
    let json: Value = serde_json::from_str::<Value>(data).or(Err("error parsing body"))?;
    let cmd = json.get("cmd").ok_or("unknown command")?;
    match cmd.as_str() {
        Some("SEND_GIFT") => {
            let value = json.get("data").ok_or("error parsing data")?;
            let uid = value.get("uid").unwrap().as_u64().unwrap();
            let uname = value.get("uname").unwrap().as_str().unwrap();
            let gift_name = value.get("giftName").unwrap().as_str().unwrap();
            let num = value.get("num").unwrap().as_u64().unwrap();
            let coin_type = value.get("coin_type").unwrap().as_str().unwrap();
            let total_coin = value.get("total_coin").unwrap().as_u64().unwrap();

            writer::write_gift(
                uid,
                uname,
                gift_name,
                num,
                coin_type,
                total_coin,
                buf,
                config.no_silver,
            );
        }
        Some("DANMU_MSG") => {
            let info = json.get("info").ok_or("error parsing info")?;
            let array = info.as_array().ok_or("error parsing danmu data")?;
            if array.len() <= 3 {
                Err("error parsing danmu data")?;
            }
            let msg = array[1].as_str().ok_or("error parsing danmu data")?;
            let uid = array[2]
                .as_array()
                .map(|user| user[0].as_u64().unwrap())
                .unwrap();
            let uname = array[2]
                .as_array()
                .map_or("unknown", |user| user[1].as_str().unwrap_or("unknown"));

            writer::write_barrage(uid, uname, msg, buf, &config.ignores, config.no_print);
        }
        _ => (),
    }

    Ok(())
}

fn decode_bufreader(bytes: &[u8]) -> io::Result<String> {
    let mut z = ZlibDecoder::new(bytes);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}