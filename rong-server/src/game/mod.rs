pub mod ball;
pub mod player;
pub mod state;

pub struct GameStateManager {
    players: Vec<player::Player>,
    balls: Vec<ball::Ball>,
    state: state::State,
}
