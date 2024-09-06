use tokio::net::UdpSocket;
use tokio::time::{Duration, Instant};
use rand::Rng;
use rong_shared::model::{NetworkPacket, ClientMessage, ServerMessage, PlayerId, GameState, Position, MovementPacket, Movement};
use rong_shared::error::ClientError;
use bincode;
use log::{info, error};

const SERVER_ADDR: &str = "127.0.0.1:2906";
const MOVE_INTERVAL: Duration = Duration::from_millis(16); // 60Hz update frequency

struct PlayerState {
    id: PlayerId,
    position: Position,
}

struct GameData {
    player: PlayerState,
    opponent_position: Position,
    ball_position: Position,
    ball_dx: f32,
    ball_dy: f32,
    last_ball_position: Position,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(SERVER_ADDR).await?;

    info!("Connected to server at {}", SERVER_ADDR);
    send_message(&socket, ClientMessage::Connect()).await?;
    info!("Sent Connect message");

    let mut game_data = GameData {
        player: PlayerState { id: PlayerId::Player1, position: (0.5, 0.0) },
        opponent_position: (0.5, 1.0),
        ball_position: (0.5, 0.5),
        ball_dx: 0.0,
        ball_dy: 0.0,
        last_ball_position: (0.5, 0.5),
    };
    let mut game_state = GameState::WaitingForPlayers;
    let mut last_move_time = Instant::now();
    let mut last_update_time = Instant::now();
    let mut sequence_number = 0;

    let mut last_state: Option<GameState> = None;

    loop {
        let mut buffer = [0; 1024];
        tokio::select! {
            // Handle incoming messages
            Ok((amt, _)) = socket.recv_from(&mut buffer) => {
                let buf = &mut [0; 1024][..amt];
                match bincode::deserialize::<NetworkPacket<ServerMessage>>(buf) {
                    Ok(packet) => {
                        info!("Received: {:?}", packet.get_payload());
                        handle_server_message(packet.get_payload(), &mut game_state, &mut game_data);
                    },
                    Err(e) => error!("Failed to deserialize packet: {:?}", e),
                }
            }

            // Update game state periodically
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                let now = Instant::now();
                let dt = now.duration_since(last_update_time).as_secs_f32();
                if dt > 0.0 {
                    game_data.ball_dx = (game_data.ball_position.0 - game_data.last_ball_position.0) / dt;
                    game_data.ball_dy = (game_data.ball_position.1 - game_data.last_ball_position.1) / dt;
                    game_data.last_ball_position = game_data.ball_position;
                    last_update_time = now;
                }

                // Send periodic moves if the game has started
                if game_state == GameState::GameStarted {
                    if last_move_time.elapsed() >= MOVE_INTERVAL {
                        send_smart_move(&socket, &game_data, &mut sequence_number).await?;
                        last_move_time = Instant::now();
                    }
                    info!("Current game state: {:?}", game_state);
                } else {
                    if last_state.is_none() || last_state != Some(game_state) {
                        info!("Current game state: {:?}", game_state);
                        last_state = Some(game_state);
                    }
                }
            }
        }
    }
}

fn handle_server_message(
    msg: &ServerMessage,
    game_state: &mut GameState,
    game_data: &mut GameData,
) {
    info!("*** Received: {:?}", msg);
    match msg {
        ServerMessage::PlayerJoined(id) => {
            game_data.player.id = *id;
            game_data.player.position.1 = if *id == PlayerId::Player1 { 0.9 } else { 0.1 };
            info!("Assigned as {:?}", id);
        }
        ServerMessage::GameStateChange(new_state) => {
            if new_state == &GameState::GameStarted {
                info!("&*****************&");
                info!("Game started RECEIVED");
                info!("&*****************&");
            }

            *game_state = *new_state;
            info!("Game state changed to {:?}", new_state);
        }
        ServerMessage::PositionUpdate(positions) => {
            let (player1, player2, ball) = positions.get_payload();
            if game_data.player.id == PlayerId::Player1 {
                game_data.player.position = *player1;
                game_data.opponent_position = *player2;
            } else {
                game_data.player.position = *player2;
                game_data.opponent_position = *player1;
            }
            game_data.ball_position = *ball;
            info!(
                "Updated game state: Player at ({:.2}, {:.2}), Opponent at ({:.2}, {:.2}), Ball at ({:.2}, {:.2})",
                game_data.player.position.0, game_data.player.position.1, 
                game_data.opponent_position.0, game_data.opponent_position.1, 
                game_data.ball_position.0, game_data.ball_position.1
            );
        }
        ServerMessage::ScoreUpdate(scores) => {
            let (score1, score2) = scores.get_payload();
            info!("Score updated: {} - {}", score1, score2);
        }
        ServerMessage::Ack(msg) => {
            info!("Server acknowledgement: {}", msg);
        }
        ServerMessage::Error(error) => {
            error!("Server error: {:?}", error);
        }
        _ => {
            info!("Unhandled message: {:?}", msg);
        }
    }
}

async fn send_message(socket: &UdpSocket, message: ClientMessage) -> Result<(), ClientError> {
    let packet = NetworkPacket::new(0, 0, message); // TODO: Implement proper sequence number and timestamp
    let serialized = bincode::serialize(&packet)?;
    socket.send(&serialized).await?;
    Ok(())
}

async fn send_smart_move(
    socket: &UdpSocket,
    game_data: &GameData,
    sequence_number: &mut u32,
) -> Result<(), ClientError> {
    let target_x = predict_ball_position(game_data);
    let distance = target_x - game_data.player.position.0;
    
    let movement = if distance.abs() < 0.01 {
        Movement::Stop
    } else if distance > 0.0 {
        Movement::Up
    } else {
        Movement::Down
    };

    let movement_packet = MovementPacket::new(game_data.player.id, movement);
    *sequence_number += 1;
    let message = ClientMessage::MovementCommand(movement_packet);
    let packet = NetworkPacket::new(*sequence_number, 0, message); // TODO: Implement proper timestamp
    let serialized = bincode::serialize(&packet)?;
    socket.send(&serialized).await?;
    info!("Sent movement: {:?}", movement);
    Ok(())
}

fn predict_ball_position(game_data: &GameData) -> f32 {
    let time_to_reach = if game_data.ball_dy != 0.0 {
        (game_data.player.position.1 - game_data.ball_position.1).abs() / game_data.ball_dy.abs()
    } else {
        0.0 // Avoid division by zero
    };
    
    let mut predicted_x = game_data.ball_position.0 + game_data.ball_dx * time_to_reach;
    
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