fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    println!("Home address is: {}", home);
}
