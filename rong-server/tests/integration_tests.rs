use rong_shared::model::{GameState, PlayerId};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

// You'll need to make sure these are publicly accessible or create public interfaces for testing
use rong_server::game::player::player_manager::PlayerManager;
use rong_server::game::state::State;

#[tokio::test]
async fn test_game_state_transitions() {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
    let player_manager = PlayerManager::new(socket);
    let mut state = State::new(player_manager);

    assert_eq!(
        state.get_state(),
        GameState::WaitingForPlayers,
        "Initial state should be WaitingForPlayers"
    );

    // Add players
    state
        .players
        .add_player(
            PlayerId::Player1,
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap(),
        )
        .await
        .unwrap();
    state
        .players
        .add_player(
            PlayerId::Player2,
            "127.0.0.1:8081".parse::<SocketAddr>().unwrap(),
        )
        .await
        .unwrap();

    // Start the game
    state.start_game().unwrap();
    assert_eq!(
        state.get_state(),
        GameState::GameStarted,
        "Game should have started"
    );

    // You might need to implement a method to end the game
    state.end_game();
    assert_eq!(
        state.get_state(),
        GameState::GameOver,
        "Game should be over"
    );
}

#[tokio::test]
async fn test_score_update() {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
    let player_manager = PlayerManager::new(socket);
    let mut state = State::new(player_manager);

    // Add players and start game
    state
        .players
        .add_player(
            PlayerId::Player1,
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap(),
        )
        .await
        .unwrap();
    state
        .players
        .add_player(
            PlayerId::Player2,
            "127.0.0.1:8081".parse::<SocketAddr>().unwrap(),
        )
        .await
        .unwrap();
    state.start_game().unwrap();

    // Initial score should be 0-0
    assert_eq!(
        state.get_scores().get_payload(),
        (0, 0),
        "Initial scores should be 0-0"
    );

    // Simulate a score (you'll need to implement a method for this)
    state.update_score(PlayerId::Player1);
    assert_eq!(
        state.get_scores().get_payload(),
        (1, 0),
        "Player 1 should have scored"
    );

    state.update_score(PlayerId::Player2);
    assert_eq!(
        state.get_scores().get_payload(),
        (1, 1),
        "Both players should have scored"
    );
}
