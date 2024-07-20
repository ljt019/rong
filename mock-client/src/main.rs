use rand::Rng;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

const SERVER_ADDR: &str = "127.0.0.1:2906";

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let mut rng = rand::thread_rng();

    socket
        .connect(SERVER_ADDR)
        .expect("connect function failed");

    socket.send(b"CONNECT").expect("send function failed");

    let mut player_id = None;
    let mut game_state = GameState::WaitingForPlayers;
    let mut last_move_time = Instant::now();

    loop {
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let received =
                    std::str::from_utf8(&buf[..amt]).expect("failed to convert to string");
                println!("Received: {}", received);

                match received {
                    "Player 1" => {
                        player_id = Some(1);
                        println!("Assigned as Player 1");
                    }
                    "Player 2" => {
                        player_id = Some(2);
                    }
                    "GAME STARTED" => {
                        game_state = GameState::GameStarted;
                        println!("Game started");
                    }
                    _ => {}
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => eprintln!("encountered IO error: {}", e),
        }

        if let GameState::GameStarted = game_state {
            if let Some(id) = player_id {
                if last_move_time.elapsed() >= Duration::from_millis(500) {
                    // Randomly choose to move left ('a') or right ('d')
                    let movement = if rng.gen_bool(0.5) { 'a' } else { 'd' };
                    let message = format!("{} {}", id, movement);

                    socket
                        .send(message.as_bytes())
                        .expect("send function failed");

                    println!("Sent: {}", message);
                    last_move_time = Instant::now();
                }
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}

enum GameState {
    WaitingForPlayers,
    GameStarted,
}
