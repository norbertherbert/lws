
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};


#[tokio::test]
async fn send_udp_msg() {

    let server_address = "0.0.0.0:8082";
    let receiver_address = "127.0.0.1:1700";

    let _ul1 = "02bbda00aaf98bf32c26b61a7b227278706b223a5b7b226368616e223a302c22636f6472223a22342f35222c2264617461223a22514f7843737757417a53514345374348566279336c7a6568222c2264617472223a225346374257313235222c2266726571223a3836372e373030303132323037303331332c226c736e72223a2d352e302c226d6f6475223a224c4f5241222c2272666368223a302c2272737369223a2d3131332c2273697a65223a31382c2273746174223a312c2274696d65223a22313937302d30312d31365432333a31343a31385a222c22746d7374223a333938343931303933317d5d7d";
    let _ul2 = "02565d00647fdafffe005e177b227278706b223a5b7b22746d7374223a3534393639333337322c226368616e223a322c2272666368223a302c2266726571223a3836372e3530303030302c2273746174223a312c226d6f6475223a224c4f5241222c2264617472223a225346394257313235222c22636f6472223a22342f35222c226c736e72223a382e322c2272737369223a2d33372c2273697a65223a31362c2264617461223a225142304d414154417541426b4f38362f6467483752513d3d227d5d7d";
    let _pull_data = "02ffff02aabbccddaabbccdd";

    let msg_hex = _ul1;

    let socket = UdpSocket::bind(server_address).await
        .expect("couldn't bind to address");


    let msg = hex::decode(msg_hex)
        .expect("couldn't decode hex encoded string");
    let num_of_bites_sent = socket.send_to(msg.as_slice(), receiver_address).await
        .expect("couldn't send data");
    println!("Number of bytes sent: {num_of_bites_sent}");


    let mut buf = [0; 1024];
    if let Ok(Ok(result)) = timeout(
        Duration::from_millis(1000), 
        socket.recv_from(&mut buf)
    ).await {
        println!("Received response: {}", hex::encode(&buf[..result.0]));
    } else {
        println!("Timeout expired");
    }

}

#[test]
fn test_hello_world() {
    let v = vec![1_u8,2,3,4,5,6,7,8,9];
    let s: &[u8] = v.as_ref();
    println!("Vector: {:?}", s);
}


fn test_01() -> Result<[u8; 4], Box<dyn std::error::Error>> {
    let a: [u8; 10] = [0,1,2,3,4,5,6,7,8,9];
    let b: &[u8] = &a[..4];
    let c: [u8; 4] = b.try_into().map_err(|e| format!("Error: {:?}", e))?;
    Ok(c)
}

#[test]
fn test_02() {
    match test_01() {
        Ok(c) => println!("{:?}", c),
        Err(e) => println!("{:?}", e),
    }
}