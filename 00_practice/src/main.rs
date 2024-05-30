#[derive(Debug)]
enum Ipaddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let localhost = Ipaddress::V4(127, 0, 0, 1);
    let exthost = Ipaddress::V6("afdfasdf::fads".to_string());
    println!("localhost is - {:?}", localhost);
    println!("exthost is - {:?}", exthost);
}
