#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    dbg!(home);

    let a = Some(6);
    dbg!(a);
    let mut b: Option<i32> = None;
    b = Some(12);
    dbg!(b);
}
