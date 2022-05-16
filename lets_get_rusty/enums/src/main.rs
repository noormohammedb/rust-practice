mod enum_advanced;
mod enum_match;

#[derive(Debug)]
enum ipVer {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct ipStruct {
    ip_ver: ipVer,
    ip_addr: String,
}

#[derive(Debug)]
struct ipAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn some_fun() {
        println!("some_fun called from Message enum impl");
    }
}

fn main() {
    let four = ipVer::V4;
    let six = ipVer::V6;
    println!("four and six are: {:#?} and {:#?}", four, six);

    let localhost_v1 = ipStruct {
        ip_ver: ipVer::V4,
        ip_addr: String::from("127.0.0.1"),
    };
    println!("locahost_v1: {:#?}", localhost_v1);

    let localhost_v2 = ipAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };
    let localhost_v3 = ipAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    println!("locahost_v2: {:?}", localhost_v2);
    println!("locahost_v3: {:?}", localhost_v3);

    enum_advanced::enum_advanced();
    enum_match::enum_match();
}
