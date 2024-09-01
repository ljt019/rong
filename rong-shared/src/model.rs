use super::error;

/*  Network packet wrapper for all messages */
pub struct NetworkPacket<T> {
    sequence: u32,
    timestamp: u64,
    payload: T,
}

impl<T> NetworkPacket<T> {
    pub fn new(sequence: u32, timestamp: u64, payload: T) -> Self {
        NetworkPacket {
            sequence,
            timestamp,
            payload,
        }
    }

    pub fn get_sequence(&self) -> u32 {
        self.sequence
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_payload(&self) -> &T {
        &self.payload
    }

    pub fn set_sequence(&mut self, sequence: u32) {
        self.sequence = sequence;
    }

    pub fn set_timesamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    pub fn set_payload(&mut self, payload: T) {
        self.payload = payload;
    }
}

pub type ClientNetworkMessage = NetworkPacket<ClientMessage>;
pub type ServerNetworkMessage = NetworkPacket<ServerMessage>;

/* Core Game enums */
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum PlayerId {
    Player1 = 0,
    Player2 = 1,
}

#[repr(u8)]
pub enum EntityId {
    Player(PlayerId),
    Ball,
}

#[repr(u8)]
pub enum Movement {
    Up = 0,
    Down = 1,
    Stop = 2,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    WaitingForPlayers,
    GameStarted,
    GameOver,
}

/*  Message enums */

// Client-to-Server message options
pub enum ClientMessage {
    Connect(PlayerId),               // Player id
    Disconnect(PlayerId),            // Player id
    MovementCommand(MovementPacket), // Player id, Movement Key
    Ack(String),                     // Acknowledgement
    Error(error::ClientError),
}

// Server-to-Client message options
pub enum ServerMessage {
    PlayerJoined(PlayerId),         // Player id
    PlayerLeft(PlayerId),           // Player id
    PositionUpdate(PositionPacket), // Player 1, Player 2, Ball
    ScoreUpdate(ScorePacket),       // Player 1 score, Player 2 score
    GameStateChange(GameState),     // New game state
    Ack(String),                    // Acknowledgement
    Error(error::ServerError),
}

pub struct Ack {
    player_id: PlayerId,
    ack_type: ServerMessage,
}

/* Packet structs */
pub struct PositionPacket {
    player1: Position,
    player2: Position,
    ball: Position,
}

impl PositionPacket {
    pub fn new(player1: Position, player2: Position, ball: Position) -> Self {
        PositionPacket {
            player1,
            player2,
            ball,
        }
    }

    pub fn get_payload(&self) -> (&Position, &Position, &Position) {
        (&self.player1, &self.player2, &self.ball)
    }
}

pub struct ScorePacket {
    player1: u8,
    player2: u8,
}

impl ScorePacket {
    pub fn new(player1: u8, player2: u8) -> Self {
        ScorePacket { player1, player2 }
    }

    pub fn get_payload(&self) -> (u8, u8) {
        (self.player1, self.player2)
    }
}

pub struct MovementPacket {
    player_id: PlayerId,
    movement: Movement,
}

impl MovementPacket {
    pub fn new(player_id: PlayerId, movement: Movement) -> Self {
        MovementPacket {
            player_id,
            movement,
        }
    }

    pub fn get_payload(&self) -> (&PlayerId, &Movement) {
        (&self.player_id, &self.movement)
    }
}

// Misc Types
pub type Position = (f32, f32);
