use rand::Rng;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

    socket
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let mut rng = rand::thread_rng();

    socket
        .connect("10.0.0.252:2906")
        .expect("connect function failed");

    socket.send(b"CONNECT").expect("send function failed");

    let mut player = None;

    let mut game_started = false;

    loop {
        let mut buf = [0; 256];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let received = &buf[..amt];
                let received_str =
                    std::str::from_utf8(received).expect("failed to convert to string");

                match received_str {
                    "PLAYER 1" => player = Some(1),
                    "PLAYER 2" => player = Some(2),
                    "GAME_START" => game_started = true,
                    _ => (),
                }

                println!("Received: {}", received_str);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available yet, so you can do other tasks here or simply continue
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }

        if game_started {
            if let Some(player_number) = player {
                // get random value between 0 and 100 for x
                let x = rng.gen_range(0..101);

                // get random value between 0 and 100 for y
                let y = rng.gen_range(0..101);

                let message = format!("{} POS {} {}", player_number, x, y);

                socket
                    .send(message.as_bytes())
                    .expect("send function failed");
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
