enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // Enum variants are namespaced under the identifier
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
