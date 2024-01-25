mod option_enum;
mod  if_let;
// #[derive(Debug)]
// enum IpAddressKind {
//     V4(String),         // we are putting data inside variant
//     V6(u8, u8, u8, u8), // it can also store 4 8-bit integers
// }
#[derive(Debug)]
enum Message {
    // these are all struct (individual struct) which can be grouped together
    Quit, //uint struct
    // Move { x: u32, y: u32 },    // struct
    Write(String),              // tuple struct
    ChangeColor(i32, i32, i32), // tuple struct
}
impl Message {
    // implementation of enum
    // fn change_m(&self, a: i32, b: i32, c: i32) -> Self {
    //     // &self is first parameter, in this case "m" or variable that method is called upon
    //     Self::ChangeColor(a, b, c) // denotes Message
    // }
}
// #[derive(Debug)]
// struct IpAddress {
//     kind: IpAddressKind, // enum type
//     address: String,
// }

fn main() {
    option_enum::option();
    if_let::if_let();
    // let mut m = Message::ChangeColor(2, 2, 2);
    // println!("{:#?}", m);
    // m = m.change_m(8, 8, 8);
    // changing the value
    // println!("changed m = {:#?}", m);
    let _quit = Message::Quit;
    // let _mov = Message::Move { x: 56, y: 56 };
    let _write = Message::Write(String::from("write"));
    let _change = Message::ChangeColor(7, 7, 7);
}
// let _four = IpAddressKind::V4; // accessing V4 using :: (namespace)
// let _six = IpAddressKind::V6;
// let localhost = IpAddress {
//     kind: IpAddressKind::V4(String::from("127.0.0.1")),
//     address: String::from("127.0.0.1"),
// };
// let localhost = IpAddressKind::V4(String::from("127.0.0.1"));

// creating instances of IpAddress struct
// let mut lh = IpAddress{
//     kind : IpAddressKind::V4(String::from("lh ip addrress")),
//     address : String::from("Hello"),
// };
// lh.address = String::from("qwerty"); // changed address
// println!("{:#?} ", localhost);
// println!("lh = {:#?}",lh);
// dbg!(lh);

// enum as parameter
// fn route(ip_kind: IpAddress) {}

// storing struct directly in enum
// standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used, but it embeds the address data inside the variants in the form of two different structs
// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
