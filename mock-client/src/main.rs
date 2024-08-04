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

struct PlayerState {
    id: u8,
    x: f32,
    y: f32,
}

struct GameData {
    player: PlayerState,
    ball_x: f32,
    ball_y: f32,
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
    let mut game_data = GameData {
        player: PlayerState {
            id: 0,
            x: 0.5,
            y: 0.0,
        },
        ball_x: 0.5,
        ball_y: 0.5,
    };
    let mut last_move_time = Instant::now();

    loop {
        // Handle incoming messages
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let received = std::str::from_utf8(&buf[..amt])?;
                println!("Received: {}", received);
                handle_server_message(received, &mut player_id, &mut game_state, &mut game_data);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => eprintln!("IO error: {}", e),
        }

        // Send periodic moves if the game has started
        if let GameState::GameStarted = game_state {
            if let Some(id) = player_id {
                if last_move_time.elapsed() >= MOVE_INTERVAL {
                    send_strategic_move(&socket, &game_data, &mut rng)?;
                    last_move_time = Instant::now();
                }
            }
        }
    }
}

fn handle_server_message(
    msg: &str,
    player_id: &mut Option<u8>,
    game_state: &mut GameState,
    game_data: &mut GameData,
) {
    match msg {
        "PLAYER 1" => {
            *player_id = Some(1);
            game_data.player.id = 1;
            println!("Assigned as Player 1");
        }
        "PLAYER 2" => {
            *player_id = Some(2);
            game_data.player.id = 2;
            println!("Assigned as Player 2");
        }
        "GAME STARTED" => {
            *game_state = GameState::GameStarted;
            println!("Game started");
        }
        _ if msg.starts_with("PLAYER") => {
            // Game state update
            let parts: Vec<&str> = msg.split_whitespace().collect();
            if parts.len() >= 9 {
                game_data.player.x = parts[1].parse().unwrap_or(game_data.player.x);
                game_data.player.y = parts[2].parse().unwrap_or(game_data.player.y);
                game_data.ball_x = parts[7].parse().unwrap_or(game_data.ball_x);
                game_data.ball_y = parts[8].parse().unwrap_or(game_data.ball_y);
                println!(
                    "Updated game state: Player at ({}, {}), Ball at ({}, {})",
                    game_data.player.x, game_data.player.y, game_data.ball_x, game_data.ball_y
                );
            }
        }
        _ => {
            println!("Unhandled message: {}", msg);
        }
    }
}

fn send_strategic_move(
    socket: &UdpSocket,
    game_data: &GameData,
    rng: &mut impl Rng,
) -> std::io::Result<()> {
    let movement = if game_data.player.x < game_data.ball_x {
        // Move right with high probability
        if rng.gen_bool(0.9) {
            'd'
        } else {
            'a'
        }
    } else if game_data.player.x > game_data.ball_x {
        // Move left with high probability
        if rng.gen_bool(0.9) {
            'a'
        } else {
            'd'
        }
    } else {
        // At the same x-position, move randomly
        if rng.gen_bool(0.5) {
            'a'
        } else {
            'd'
        }
    };

    let message = format!("{} {}", game_data.player.id, movement);
    socket.send(message.as_bytes())?;
    println!("Sent: {}", message);
    Ok(())
}
