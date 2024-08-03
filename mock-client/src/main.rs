use rand::Rng;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

const SERVER_ADDR: &str = "127.0.0.1:2906";
const MOVE_INTERVAL: Duration = Duration::from_millis(100); // Send a move every 100ms

#[derive(Debug)]
enum GameState {
    WaitingForPlayers,
    GameStarted,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_nonblocking(true)?;
    socket.connect(SERVER_ADDR)?;

    println!("Connected to server at {}", SERVER_ADDR);
    socket.send(b"CONNECT")?;
    println!("Sent CONNECT message");

    let mut rng = rand::thread_rng();
    let mut player_id = None;
    let mut game_state = GameState::WaitingForPlayers;
    let mut last_move_time = Instant::now();

    loop {
        // Handle incoming messages
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let received = std::str::from_utf8(&buf[..amt])?;
                println!("Received: {}", received);
                handle_server_message(received, &mut player_id, &mut game_state);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => eprintln!("IO error: {}", e),
        }

        // Send periodic moves if the game has started
        if let GameState::GameStarted = game_state {
            if let Some(id) = player_id {
                if last_move_time.elapsed() >= MOVE_INTERVAL {
                    send_random_move(&socket, id, &mut rng)?;
                    last_move_time = Instant::now();
                }
            }
        }
    }
}

fn handle_server_message(msg: &str, player_id: &mut Option<u8>, game_state: &mut GameState) {
    match msg {
        "PLAYER 1" => {
            *player_id = Some(1);
            println!("Assigned as Player 1");
        }
        "PLAYER 2" => {
            *player_id = Some(2);
            println!("Assigned as Player 2");
        }
        "GAME STARTED" => {
            *game_state = GameState::GameStarted;
            println!("Game started");
        }
        _ if msg.starts_with("PLAYER") => {
            // Game state update, print for debugging
            println!("Game state update: {}", msg);
        }
        _ => {
            println!("Unhandled message: {}", msg);
        }
    }
}

fn send_random_move(socket: &UdpSocket, id: u8, rng: &mut impl Rng) -> std::io::Result<()> {
    let movement = if rng.gen_bool(0.5) { 'a' } else { 'd' };
    let message = format!("{} {}", id, movement);
    socket.send(message.as_bytes())?;
    println!("Sent: {}", message);
    Ok(())
}
