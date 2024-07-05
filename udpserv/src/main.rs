use tokio::net::UdpSocket;
use std::{io, str::from_utf8};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:1234").await?;  // Listen on all interfaces at port 1234
    let mut buf = [0; 1024];  // Buffer to hold incoming data

    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;  // Receive data into the buffer
        println!("Received from {:?}", addr);  // Log the source address

        // Ensure there is enough data to read joystick values
        if len >= 17 {  // Check if the buffer has enough bytes to read data for two joysticks
            // Decode joystick data for the first joystick
            let lx1 = i16::from_ne_bytes([buf[9], buf[10]]);
            let ly1 = i16::from_ne_bytes([buf[11], buf[12]]);
            println!("Joystick 1 - LX: {}, LY: {}", lx1, ly1);

            // Decode joystick data for the second joystick
            let lx2 = i16::from_ne_bytes([buf[13], buf[14]]);
            let ly2 = i16::from_ne_bytes([buf[15], buf[16]]);
            println!("Joystick 2 - LX: {}, LY: {}", lx2, ly2);
        }
    }
}
