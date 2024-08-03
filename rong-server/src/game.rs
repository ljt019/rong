/*

This is the Game module. It contains the Game struct and its implementation.

The game is a simple struct that only has 4 fields:
- players: A hashmap of players connected to the game
- ball: The ball object used in the game
- state: The current state of the game
- socket: The UDP socket used to communicate with the players

The Game struct has the following methods:
- new: Creates a new Game with default values
- get_state: Gets the current state of the game
- connect_player: Connects a player to the game
- start_game: Starts the game when 2 players are connected
- waiting_for_players: Waits for players to connect to the game
- game_loop: The main game loop that updates the game state
- update_game_state: Updates the game state based on player input
- send_game_state: Sends the updated game state to all players

The Game struct uses the Ball and Player structs to manage the game state and player interactions.

*/

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

        // Get the number of players currently connected
        let player_count = self.players.len();

        // If there are less than 2 players connected
        if player_count < 2 {
            // Clone the socket to send messages to the player
            let socket_clone = self.socket.try_clone()?;

            // Create a new player with the next ID
            let player = Player::new(player_count as u8 + 1, src, socket_clone);

            // Insert the player into the players hashmap
            self.players.insert(player.get_id(), player);

            // Get the player by ID
            let player = self.players.get(&(player_count as u8 + 1)).unwrap();

            // format message to be sent to the player, notifying them of their player number
            let response = if player_count == 0 {
                "PLAYER 1"
            } else {
                "PLAYER 2"
            };

            // send the message to the player
            player.send(response)?;

            // If there are 2 players connected start the game
            if self.players.len() == 2 {
                self.start_game();
            }
        }

        Ok(())
    }

    fn start_game(&mut self) {
        println!("Game starting");

        // Set the ball position to the center of the screen
        self.ball.set_position(0.5, 0.5); // Center of the screen

        // Set the player positions
        for (id, player) in self.players.iter_mut() {
            // If the player ID is 1, set the player to the bottom of the screen (10% from bottom, horizontally centered)
            // If the player ID is 2, set the player to the top of the screen ( 10% from top, horizontally centered)
            if *id == 1 {
                player.update_position(0.5, 0.9);
            } else if *id == 2 {
                player.update_position(0.5, 0.1);
            }
        }

        // Send game started message to all connected players
        for player in self.players.values() {
            player.send("GAME STARTED").unwrap();
        }

        self.state = GameState::GameStarted;
    }

    pub fn waiting_for_players(&mut self) -> io::Result<()> {
        println!("Waiting for players");

        // Create buffer to store incoming messages
        let mut buf = [0; 1024];

        // Receive messages from players
        match self.socket.recv_from(&mut buf) {
            // If a message was received
            Ok((amt, src)) => {
                // Convert the message to a string
                let msg = std::str::from_utf8(&buf[..amt])
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                match msg {
                    "CONNECT" => {
                        // Connect the player
                        self.connect_player(src)?;
                    }
                    _ => {} // Ignore invalid messages
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn game_loop(&mut self) -> io::Result<()> {
        // Create buffer to store incoming messages
        let mut buf = [0; 1024];

        // Receive messages from players
        match self.socket.recv_from(&mut buf) {
            // If a message was received
            Ok((amt, _src)) => {
                // Convert the message to a string
                let msg = std::str::from_utf8(&buf[..amt])
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                // Convert the message to lowercase
                let msg = msg.to_lowercase();

                // Split the message into parts
                let parts: Vec<&str> = msg.split_whitespace().collect();

                // If the message has 2 parts
                if parts.len() == 2 {
                    // Parse the player ID
                    if let Ok(id) = parts[0].parse::<u8>() {
                        // Get the player by ID
                        if let Some(player) = self.players.get_mut(&id) {
                            // Move the player based on the message
                            match parts[1] {
                                "a" => player.move_left(),
                                "d" => player.move_right(),
                                _ => {} // Ignore invalid movement commands
                            }
                        }
                    }
                }

                // Update the game state
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
        // Send game state to all players
        for (id, player) in &self.players {
            // let the opponents_id be the other player (only 2 players)
            let opponent_id = if *id == 1 { 2 } else { 1 };

            // get x and y position using the player get_position method
            let (player_x, player_y) = player.get_position();

            // get the opponent by player id then use the player get_position method
            let (opponent_x, opponent_y) = self.players.get(&opponent_id).unwrap().get_position();

            // get the ball x and y position using the ball get_position method
            let (ball_x, ball_y) = self.ball.get_position();

            // format the message to be sent to the player
            let msg = format!(
                "PLAYER {player_x} {player_y} OPPONENT {opponent_x} {opponent_y} BALL {ball_x} {ball_y}",
            );

            // send the message to the player
            player.send(&msg)?;
        }

        Ok(())
    }
}
