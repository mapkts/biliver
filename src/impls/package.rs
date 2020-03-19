#[derive(Debug)]
pub struct Package {
    pub length: usize,
    pub version: u32,
    /// actions:
    /// * 2 - heartbeat
    /// * 3 - popularity
    /// * 5 - notification
    /// * 7 - request
    /// * 8 - response
    pub action: u32,
    pub param: u32,
    pub body: Option<String>,
}

impl Package {
    pub fn new() -> Self {
        Package {
            length: 0x0000_0010,
            version: 0x0010_0001,
            action: 0x0000_0000,
            param: 0x0000_0001,
            body: None,
        }
    }

    pub fn set_body(&mut self, body: Option<String>) {
        match body {
            Some(body) => {
                self.length = body.as_bytes().len() + 16;
                self.body = Some(body);
            },
            None => {
                self.length = 0x0000_0010;
                self.body = None;
            }
        }
    }

    pub fn join_room(user_id: u32, room_id: u32) -> Self {
        let mut package = Package::new();
        package.action = 7;
        let body = format!("{{\"roomid\":{},\"uid\":{}}}", room_id, user_id);
        package.set_body(Some(body));
        package
    }

    pub fn heartbeat() -> Self {
        Package {
            length: 0x0000_0010,
            version: 0x0010_0001,
            action: 0x0000_0002,
            param: 0x0000_0001,
            body: None,
        }
    }
}