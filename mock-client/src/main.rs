use rand::Rng;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

const SERVER_ADDR: &str = "127.0.0.1:2906";
const MOVE_INTERVAL: Duration = Duration::from_millis(16); // 60Hz update frequency

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
    opponent_x: f32,
    opponent_y: f32,
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    last_ball_x: f32,
    last_ball_y: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_nonblocking(true)?;
    socket.connect(SERVER_ADDR)?;

    println!("Connected to server at {}", SERVER_ADDR);
    socket.send(b"CONNECT")?;
    println!("Sent CONNECT message");

    let mut player_id = None;
    let mut game_state = GameState::WaitingForPlayers;
    let mut game_data = GameData {
        player: PlayerState { id: 0, x: 0.5, y: 0.0 },
        opponent_x: 0.5,
        opponent_y: 1.0,
        ball_x: 0.5,
        ball_y: 0.5,
        ball_dx: 0.0,
        ball_dy: 0.0,
        last_ball_x: 0.5,
        last_ball_y: 0.5,
    };
    let mut last_move_time = Instant::now();
    let mut last_update_time = Instant::now();

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

        // Update ball velocity
        let now = Instant::now();
        let dt = now.duration_since(last_update_time).as_secs_f32();
        if dt > 0.0 {
            game_data.ball_dx = (game_data.ball_x - game_data.last_ball_x) / dt;
            game_data.ball_dy = (game_data.ball_y - game_data.last_ball_y) / dt;
            game_data.last_ball_x = game_data.ball_x;
            game_data.last_ball_y = game_data.ball_y;
            last_update_time = now;
        }

        // Send periodic moves if the game has started
        if let GameState::GameStarted = game_state {
            if player_id.is_some() && last_move_time.elapsed() >= MOVE_INTERVAL {
                send_smart_move(&socket, &game_data)?;
                last_move_time = Instant::now();
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
            game_data.player.y = 0.9; // Assuming player 1 is at the bottom
            println!("Assigned as Player 1");
        }
        "PLAYER 2" => {
            *player_id = Some(2);
            game_data.player.id = 2;
            game_data.player.y = 0.1; // Assuming player 2 is at the top
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
                game_data.opponent_x = parts[4].parse().unwrap_or(game_data.opponent_x);
                game_data.opponent_y = parts[5].parse().unwrap_or(game_data.opponent_y);
                game_data.ball_x = parts[7].parse().unwrap_or(game_data.ball_x);
                game_data.ball_y = parts[8].parse().unwrap_or(game_data.ball_y);
                println!(
                    "Updated game state: Player at ({}, {}), Opponent at ({}, {}), Ball at ({}, {})",
                    game_data.player.x, game_data.player.y, 
                    game_data.opponent_x, game_data.opponent_y, 
                    game_data.ball_x, game_data.ball_y
                );
            }
        }
        _ => {
            println!("Unhandled message: {}", msg);
        }
    }
}

fn send_smart_move(
    socket: &UdpSocket,
    game_data: &GameData,
) -> std::io::Result<()> {
    let target_x = predict_ball_position(game_data);
    let distance = target_x - game_data.player.x;
    
    let movement = if distance.abs() < 0.01 {
        // If very close to the target, don't move
        'x'
    } else if distance > 0.0 {
        'd' // Move right
    } else {
        'a' // Move left
    };

    let message = format!("{} {}", game_data.player.id, movement);
    socket.send(message.as_bytes())?;
    println!("Sent: {}", message);
    Ok(())
}

fn predict_ball_position(game_data: &GameData) -> f32 {
    let time_to_reach = if game_data.ball_dy != 0.0 {
        (game_data.player.y - game_data.ball_y).abs() / game_data.ball_dy.abs()
    } else {
        0.0 // Avoid division by zero
    };
    
    let mut predicted_x = game_data.ball_x + game_data.ball_dx * time_to_reach;
    
    // Handle bounces off side walls
    while predicted_x < 0.0 || predicted_x > 1.0 {
        if predicted_x < 0.0 {
            predicted_x = -predicted_x;
        } else if predicted_x > 1.0 {
            predicted_x = 2.0 - predicted_x;
        }
    }
    
    // Add some randomness to make the AI less predictable
    let mut rng = rand::thread_rng();
    predicted_x + rng.gen_range(-0.05..0.05)
}