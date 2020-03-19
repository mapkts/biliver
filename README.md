# bilive-helper
Bilibili直播弹幕、人气、礼物打印收集工具 (支持linux和windows双平台)

### 安装方法
(请确保你的机器安装了[Rust](https://www.rust-lang.org/tools/install))
```
git clone https://github.com/mapkts/bilibili-helper.git
cd bilibili-helper && cargo build --release && cp ./target/release/bilive ./ && rm -rf ./target
```

### 使用方法
* 首次使用需修改项目根目录下的配置文件conf.toml, 可配置项说明如下：

| 配置项 | 默认值 | 说明 |
| ----- | ----- | ----- |
| user_id | | 用户id，此项修改为自己的B站直播uid |
| room_id | | 房间号，可从房间页面url获取 |
| no_print | false | 默认打印格式化的弹幕到控制台 |
| log_interval | 3 | 每两次写入人气值的时间间隔，单位为分钟 |
| log_threshold | 10 | 人气阈值，低于阈值的人气不会写入日志 |
| ignores | | 忽略特定用户的弹幕，弹幕不会打印到控制台和写入日志文件 |
| no_silver | true | 默认不写入银瓜子礼物记录（辣条、免费道具等) |

* 以后每次使用只需执行命令：`./bilive`
（`./bilive --help`可查看可选命令行参数，**命令行传递的参数会覆盖配置文件的参数**）

### 数据写入
人气、弹幕、礼物数据默认按时间顺序写入到当前工作目录下的`[room_id].log.csv`文件中，默认编码为GB18030，经测试在Excel和windows记事本下可正常显示（无乱码）

各项写入数据对照说明如下：
```
Popularity,2020-03-19,14:08:04,8486
人气，日期，时间，人气值
```
```
Barrage,2020-03-19,14:08:44,10269709,"一个B站用户","妈妈问我为什么跪着看直播 w(??Д??)w"
弹幕，日期，时间，用户id，用户名，弹幕内容
```
```
Gift,2020-03-19,14:49:25,378088413,"一个B站用户","摩天大楼",1,360000,gold
礼物，日期，时间，用户id，用户名，礼物名，数量，总瓜子，瓜子类型
```

### 弹幕打印
(打印顺序：弹幕时间 发送用户 弹幕内容)
![Screenshot](https://github.com/mapkts/bilibili-helper/raw/master/demo/screenshot.png)


一个简单的小工具，觉得还不错就给个star吧 (  •̆ ᵕ •̆ )◞♡