#[derive(Debug)]
enum IpAddressKind {
    V4(String),         // we are putting data inside variant
    V6(u8, u8, u8, u8), // it can also store 4 8-bit integers
}
enum Message {
    // these are all struct (individual struct) which can be grouped together
    Quit, //uint struct
    Move { x: u32, y: u32 },
    Write(String), // tuple struct
    ChangeColor(i32, i32, i32), // tuple struct
}
impl Message {
    fn some_function(){
        println!("Lets Go Rust!!!");
    }
}
struct IpAddress {
    kind: IpAddressKind, // enum type
    address: String,
}
fn main() {
    let _four = IpAddressKind::V4; // accessing V4 using :: (namespace)
    let _six = IpAddressKind::V6;
    // let localhost = IpAddress {
    //     kind: IpAddressKind::V4(String::from("127.0.0.1")),
    //     address: String::from("127.0.0.1"),
    // };
    let localhost = IpAddressKind::V4((String::from("127.0.0.1")));
    println!("{:#?}", localhost);
}

// enum as parameter
// fn route(ip_kind: IpAddress) {}
