use crate::ball::Ball;
use crate::players::Player;

use std::collections::HashMap;
use std::io;
use std::net::UdpSocket;

#[derive(Clone, Copy)]
pub enum GameState {
    WaitingForPlayers,
    GameStarted,
}

pub struct Game {
    players: HashMap<u8, Player>,
    ball: Ball,
    state: GameState,
    socket: UdpSocket,
}

impl Game {
    pub fn new(socket: UdpSocket) -> Self {
        Self {
            players: HashMap::new(),
            ball: Ball::new(),
            state: GameState::WaitingForPlayers,
            socket,
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    fn connect_player(&mut self, src: std::net::SocketAddr) -> io::Result<()> {
        println!("Player connected: {:?}", src);

        let player_count = self.players.len();

        if player_count < 2 {
            let socket_clone = self.socket.try_clone()?;
            let player = Player::new(player_count as u8 + 1, src, socket_clone);

            self.players.insert(player.get_id(), player);

            let player = self.players.get(&(player_count as u8 + 1)).unwrap();

            let response = if player_count == 0 {
                "PLAYER 1"
            } else {
                "PLAYER 2"
            };

            player.send(response)?;

            if self.players.len() == 2 {
                self.start_game();
            }
        }

        Ok(())
    }

    fn start_game(&mut self) {
        println!("Game starting");

        self.ball.set_position(0.5, 0.5); // Center of the screen

        for (id, player) in self.players.iter_mut() {
            if *id == 1 {
                player.update_position(0.1, 0.5); // 10% from left, vertically centered
            } else {
                player.update_position(0.9, 0.5); // 10% from right, vertically centered
            }
        }

        // Send game started message to players
        for player in self.players.values() {
            player.send("GAME STARTED").unwrap();
        }

        self.state = GameState::GameStarted;
    }

    pub fn waiting_for_players(&mut self) -> io::Result<()> {
        println!("Waiting for players");

        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let msg = std::str::from_utf8(&buf[..amt])
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                if msg == "CONNECT" {
                    self.connect_player(src)?;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn game_loop(&mut self) -> io::Result<()> {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                let msg = std::str::from_utf8(&buf[..amt])
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                let msg = msg.to_lowercase();

                let parts: Vec<&str> = msg.split_whitespace().collect();
                if parts.len() == 2 {
                    if let Ok(id) = parts[0].parse::<u8>() {
                        if let Some(player) = self.players.get_mut(&id) {
                            match parts[1] {
                                "a" => player.move_left(),
                                "d" => player.move_right(),
                                _ => {} // Ignore invalid movement commands
                            }
                        }
                    }
                }

                self.update_game_state();
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Even if no input, still update the game state
                self.update_game_state();
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn update_game_state(&mut self) {
        // Move the ball
        self.ball.move_ball();

        // Check for collisions with players
        for player in self.players.values() {
            if self.ball.collides_with_player(player) {
                self.ball.bounce_off_player(player);
            }
        }

        // Check for collisions with walls
        if self.ball.collides_with_wall() {
            self.ball.bounce_off_wall();
        }

        // Send updated game state to all players
        self.send_game_state().expect("Failed to send game state");
    }

    fn send_game_state(&self) -> io::Result<()> {
        for (id, player) in &self.players {
            let opponent_id = if *id == 1 { 2 } else { 1 };
            let (x, y) = player.get_position();
            let (op_x, op_y) = self.players.get(&opponent_id).unwrap().get_position();
            let (ball_x, ball_y) = self.ball.get_position();

            let msg = format!(
                "PLAYER {} {} OPPONENT {} {} BALL {} {}",
                x, y, op_x, op_y, ball_x, ball_y
            );

            player.send(&msg)?;
        }

        Ok(())
    }
}
