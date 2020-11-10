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

    let x: Option<i32> = Some(6);
    let y: Option<i32> = Some(5);
    let z: Option<i32> = Some(3);

    let result = plus_one(z);
    println!("Результат: {:?}", result);

    if let Some(3) = z {
        println!("three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
