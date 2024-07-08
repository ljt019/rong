use macroquad::prelude::*;
use std::net::UdpSocket;

const PLAYER_WIDTH: f32 = 100.0;
const PLAYER_HEIGHT: f32 = 10.0;

struct Player {
    id: u8,
    x: f32,
    y: f32,
}

impl Player {
    fn new(id: u8, x: f32, y: f32) -> Self {
        Player { id, x, y }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, PLAYER_WIDTH, PLAYER_HEIGHT, GREEN);
    }

    fn move_left(&mut self) {
        self.x = (self.x - 1.0).max(0.0);
    }

    fn move_right(&mut self) {
        self.x = (self.x + 1.0).min(screen_width() - PLAYER_WIDTH);
    }

    fn move_up(&mut self) {
        self.y = (self.y - 1.0).max(0.0);
    }

    fn move_down(&mut self) {
        self.y = (self.y + 1.0).min(screen_height() - PLAYER_HEIGHT);
    }
}

struct Opponent {
    x: f32,
    y: f32,
}

impl Opponent {
    fn new(x: f32, y: f32) -> Self {
        Opponent { x, y }
    }

    fn draw(&self) {
        // x and y need to be on opposite side of screen from player (i.e mirrored vertically)
        draw_rectangle(self.x, self.y, PLAYER_WIDTH, PLAYER_HEIGHT, RED);
    }

    fn update_position(&mut self, x: f32, y: f32) {
        self.x = screen_width() - x - PLAYER_WIDTH;
        self.y = screen_height() - y - PLAYER_HEIGHT;
    }
}

#[macroquad::main("Pong Client")]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind UDP socket");
    socket
        .set_nonblocking(true)
        .expect("Could not set UDP socket to non-blocking");

    let send_port = socket.local_addr().unwrap().port();
    println!("Client bound to port: {}", send_port);

    println!("Attempting to send connect message to server...");
    let connect_message = "CONNECT";
    match socket.send_to(connect_message.as_bytes(), "192.168.1.75:2906") {
        Ok(_) => println!("Connect message sent successfully"),
        Err(e) => println!("Failed to send connect message: {}", e),
    }

    let screen_width = screen_width();
    let screen_height = screen_height();

    let mut player = Player::new(
        0,
        screen_width / 2.0 - PLAYER_WIDTH / 2.0,
        screen_height - PLAYER_HEIGHT - 10.0,
    );

    // Opponent starts at the top of the screen
    let mut opponent = Opponent::new(screen_width / 2.0 - PLAYER_WIDTH / 2.0, 10.0);

    let mut game_state = GameState::WaitingForPlayer2;

    let mut last_x = player.x;
    let mut last_y = player.y;

    loop {
        clear_background(BLACK);

        // Draw player and opponent
        player.draw();
        opponent.draw();

        // Handle network communication and update game state
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let received = std::str::from_utf8(&buf[..amt]).unwrap();
                match received {
                    "PLAYER 1" => {
                        player.id = 1;
                        game_state = GameState::WaitingForPlayer2;
                        println!("Received message: Player 1 connected");
                    }
                    "PLAYER 2" => {
                        player.id = 2;
                        game_state = GameState::WaitingForGameStart;
                        println!("Received message: Player 2 connected");
                    }
                    "GAME_START" => {
                        game_state = GameState::GameRunning;
                        println!("Received message: Game starting");
                    }
                    _ => {
                        println!("Received message: {}", received);

                        let received_parts: Vec<&str> = received.split_whitespace().collect();

                        let id = received_parts[0].parse::<u8>().unwrap();
                        let x = received_parts[2].parse::<f32>().unwrap();
                        let y = received_parts[3].parse::<f32>().unwrap();

                        if id == player.id {
                            continue;
                        } else {
                            opponent.update_position(x, y);
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => eprintln!("Error receiving data: {}", e),
        }

        // Render game state text based on current game state
        match game_state {
            GameState::WaitingForPlayer2 => {
                draw_text("Waiting for Player 2", 20.0, 20.0, 30.0, WHITE);
            }
            GameState::WaitingForGameStart => {
                draw_text("Waiting for Game Start", 20.0, 60.0, 30.0, WHITE);
            }
            GameState::GameRunning => {
                // Player movement controls
                if is_key_down(KeyCode::Right) {
                    player.move_right();
                }
                if is_key_down(KeyCode::Left) {
                    player.move_left();
                }
                if is_key_down(KeyCode::Down) {
                    player.move_down();
                }
                if is_key_down(KeyCode::Up) {
                    player.move_up();
                }

                // Send player position to server if changed
                if last_x != player.x || last_y != player.y {
                    let message = format!("{} POS {} {}", player.id, player.x, player.y);
                    socket
                        .send_to(message.as_bytes(), "192.168.1.75:2906")
                        .expect("Could not send message");
                }

                last_x = player.x;
                last_y = player.y;
            }
        }

        // Check for escape key press to exit loop
        if is_key_down(KeyCode::Escape) {
            break;
        }

        // Proceed to next frame
        next_frame().await;
    }
}

enum GameState {
    WaitingForPlayer2,
    WaitingForGameStart,
    GameRunning,
}
