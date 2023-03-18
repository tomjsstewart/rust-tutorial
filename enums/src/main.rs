enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// We can use structs to bundle up other info with the enum, but it is better to store it within the enum
// enum IpAddrKind {
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind: IpAddrKind,
//     addres: String,
// }

// fn main() {
//     // Enum variants are namespaced under the identifier
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address : String::from("127.0.0.1")
//     }

//     let loopback IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1")
//     }
// }

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::0"));
}
