// fn main() -> () {
//     const IMAGE_WIDTH: u64 = 256;
//     const IMAGE_HEIGHT: u64 = 256;

//     println!("P3");
//     println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
//     println!("255");

//     for j in 0..IMAGE_HEIGHT {
//         for i in 0..IMAGE_WIDTH {
//             let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
//             let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
//             let b = 0.25;

//             let ir = (255.999 * r) as u64;
//             let ig = (255.999 * g) as u64;
//             let ib = (255.999 * b) as u64;

//             println!("{} {} {}", ir, ig, ib);
//         }
//     }
// }

use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3000")?;

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and print msg.
    let buf = &mut buf[..amt];
    let msg = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", &msg);
    socket.send_to(msg.as_bytes(), "127.0.0.1:5000")?;
    socket.send_to(buf, "127.0.0.1:5000")?;

    Ok(())
}
