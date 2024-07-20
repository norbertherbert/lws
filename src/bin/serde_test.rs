use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct UdpServer {
    #[serde(default = "UdpServer::default_addr")]
    pub addr: String,
}
impl UdpServer {
    fn default_addr() -> String { "2.2.2.2:1700".to_string() }
}
impl Default for UdpServer {
    fn default() -> Self { 
        UdpServer{
            addr: UdpServer::default_addr()
        } 
    }
}

fn main() {

    let data = r#"
    {
        "addr1": "1.1.1.1:1700"
    }"#;

    let x: UdpServer = Default::default();

    let y: UdpServer = serde_json::from_str(data).unwrap();

    println!("Data: {:?}", data);
    println!("Struct_Default: {:?}", x);
    println!("Serde_Result: {:?}", y);

}