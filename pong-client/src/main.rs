use macroquad::prelude::*;
use std::net::UdpSocket;

const SERVER_ADDR: &str = "192.168.1.75:2906";

const PLAYER_WIDTH: f32 = 100.0;
const PLAYER_HEIGHT: f32 = 10.0;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct Player {
    id: u8,
    x: f32,
    y: f32,
}

impl Player {
    fn new(id: u8) -> Self {
        Player { id, x: 0.0, y: 0.0 }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        draw_rectangle(
            self.x * SCREEN_WIDTH,
            self.y * SCREEN_HEIGHT,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            GREEN,
        );
    }

    fn move_left(&mut self) {
        self.x = (self.x - 0.01).max(0.0);
    }

    fn move_right(&mut self) {
        self.x = (self.x + 0.01).min(1.0 - PLAYER_WIDTH / SCREEN_WIDTH);
    }
}

struct Opponent {
    x: f32,
    y: f32,
}

impl Opponent {
    fn new() -> Self {
        Opponent { x: 0.0, y: 0.0 }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        draw_rectangle(
            self.x * SCREEN_WIDTH,
            self.y * SCREEN_HEIGHT,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            RED,
        );
    }
}

struct Ball {
    x: f32,
    y: f32,
}

impl Ball {
    fn new() -> Self {
        Ball { x: 0.5, y: 0.5 }
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn draw(&self) {
        draw_circle(self.x * SCREEN_WIDTH, self.y * SCREEN_HEIGHT, 10.0, WHITE);
    }
}

enum GameState {
    WaitingForPlayers,
    GameStarted,
}

#[macroquad::main("Pong Client")]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind UDP socket");
    socket
        .set_nonblocking(true)
        .expect("Could not set UDP socket to non-blocking");

    println!(
        "Client bound to port: {}",
        socket.local_addr().unwrap().port()
    );

    socket
        .send_to(b"CONNECT", SERVER_ADDR)
        .expect("Failed to send connect message");

    let mut player = Player::new(0);
    let mut opponent = Opponent::new();
    let mut ball = Ball::new();
    let mut game_state = GameState::WaitingForPlayers;
    let mut last_move_time = get_time();

    loop {
        clear_background(BLACK);

        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let received = std::str::from_utf8(&buf[..amt]).unwrap();
                println!("Received: {}", received); // Debug print

                if received.starts_with("PLAYER ") {
                    let parts: Vec<&str> = received.split_whitespace().collect();
                    if parts.len() >= 9 {
                        player.set_position(parts[1].parse().unwrap(), parts[2].parse().unwrap());
                        opponent.set_position(parts[4].parse().unwrap(), parts[5].parse().unwrap());
                        ball.set_position(parts[7].parse().unwrap(), parts[8].parse().unwrap());
                        game_state = GameState::GameStarted;
                    }
                } else if received == "PLAYER 1" || received == "PLAYER 2" {
                    player.id = received.split_whitespace().nth(1).unwrap().parse().unwrap();
                    println!("Assigned as Player {}", player.id);
                } else if received == "GAME STARTED" {
                    game_state = GameState::GameStarted;
                    println!("Game started");
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => eprintln!("Error receiving data: {}", e),
        }

        match game_state {
            GameState::WaitingForPlayers => {
                draw_text("Waiting for players...", 20.0, 20.0, 30.0, WHITE);
            }
            GameState::GameStarted => {
                player.draw();
                opponent.draw();
                ball.draw();

                if is_key_down(KeyCode::Left) {
                    if get_time() - last_move_time > 0.05 {
                        socket
                            .send_to(format!("{} a", player.id).as_bytes(), SERVER_ADDR)
                            .expect("Failed to send move");
                        last_move_time = get_time();
                    }
                }
                if is_key_down(KeyCode::Right) {
                    if get_time() - last_move_time > 0.05 {
                        socket
                            .send_to(format!("{} d", player.id).as_bytes(), SERVER_ADDR)
                            .expect("Failed to send move");
                        last_move_time = get_time();
                    }
                }
            }
        }

        // Debug information
        draw_text(
            &format!("Player: ({:.2}, {:.2})", player.x, player.y),
            10.0,
            SCREEN_HEIGHT - 60.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Opponent: ({:.2}, {:.2})", opponent.x, opponent.y),
            10.0,
            SCREEN_HEIGHT - 40.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Ball: ({:.2}, {:.2})", ball.x, ball.y),
            10.0,
            SCREEN_HEIGHT - 20.0,
            20.0,
            WHITE,
        );

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
