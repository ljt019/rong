use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

const SERVER_ADDR: &str = "0.0.0.0:2906";

struct Player {
    id: u8,
    addr: SocketAddr,
}

impl Player {
    fn new(id: u8, addr: SocketAddr) -> Self {
        println!("Player struct created: {}", addr);
        Self { id, addr }
    }
}

enum GameState {
    WaitingForPlayers,
    GameStarted,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(SERVER_ADDR)?;
    socket.set_nonblocking(true)?;

    let mut players: HashMap<u8, Player> = HashMap::new();
    let mut game_state = GameState::WaitingForPlayers;

    println!("Server listening on {}", SERVER_ADDR);
    println!("Waiting for players...");

    loop {
        match game_state {
            GameState::WaitingForPlayers => {
                handle_waiting_for_players(&socket, &mut players, &mut game_state)?;
            }
            GameState::GameStarted => {
                handle_game_started(&socket, &players)?;
            }
        }
    }
}

fn handle_waiting_for_players(
    socket: &UdpSocket,
    players: &mut HashMap<u8, Player>,
    game_state: &mut GameState,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((amt, src)) => {
            let msg = std::str::from_utf8(&buf[..amt])?;
            println!("Received message from {}: {}", src, msg);

            match msg {
                "CONNECT" => {
                    if players.len() < 2 {
                        let player_count = players.len();
                        let id = player_count as u8 + 1;
                        let player = Player::new(id, src);
                        players.insert(id, player);

                        let response = if player_count == 0 {
                            "PLAYER 1"
                        } else {
                            "PLAYER 2"
                        };
                        socket.send_to(response.as_bytes(), src)?;

                        if players.len() == 2 {
                            println!("Game starting");
                            for player in players.values() {
                                let start_msg = "GAME_START";
                                socket.send_to(start_msg.as_bytes(), player.addr)?;
                            }
                            *game_state = GameState::GameStarted;
                        } else {
                            println!("Waiting for player 2...");
                        }
                    }
                }
                _ => {}
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No data available yet, so we can do other tasks here if needed
        }
        Err(e) => return Err(e.into()), // An actual error occurred
    }
    Ok(())
}

fn handle_game_started(
    socket: &UdpSocket,
    players: &HashMap<u8, Player>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    match socket.recv_from(&mut buf) {
        Ok((amt, src)) => {
            let msg = std::str::from_utf8(&buf[..amt])?;
            let mut received = msg.split_whitespace();
            let id: u8 = received.next().unwrap().parse().unwrap();

            println!("Received message from Player_{}: {}", id, msg);

            // Forward the message to the opponent
            let opponent_id = if id == 1 { 2 } else { 1 };

            if let Some(opponent) = players.get(&opponent_id) {
                socket.send_to(msg.as_bytes(), opponent.addr)?;
            }

            println!(
                "Forwarded message from Player_{} to Player_{}",
                id, opponent_id
            );
            println!("");
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No data available yet, so we can do other tasks here if needed
        }
        Err(e) => return Err(e.into()), // An actual error occurred
    }
    Ok(())
}
