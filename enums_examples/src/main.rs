use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let x: Option<i8> = Some(6);
    let y: Option<i8> = Some(5);
}
