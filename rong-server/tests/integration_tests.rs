use rong_shared::model::{ClientMessage, GameState, NetworkPacket, PlayerId, ServerMessage};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, timeout, Duration};

use rong_server::game::player::player_manager::PlayerManager;
use rong_server::game::state::State;
use rong_server::network::connection::ConnectionManager;

async fn setup_test_environment() -> (
    State,
    mpsc::Sender<(NetworkPacket<ClientMessage>, SocketAddr)>,
    Arc<UdpSocket>,
    Arc<Mutex<ConnectionManager>>,
) {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
    let player_manager = PlayerManager::new(Arc::clone(&socket));
    let state = State::new(player_manager);

    // Create a channel for the ConnectionManager
    let (tx, _rx) = mpsc::channel(100);

    // Create the ConnectionManager
    let connection_manager = Arc::new(Mutex::new(
        ConnectionManager::new(Arc::clone(&socket), tx.clone())
            .await
            .unwrap(),
    ));

    // Spawn the ConnectionManager's run loop
    let cm_clone = Arc::clone(&connection_manager);
    tokio::spawn(async move {
        let mut cm = cm_clone.lock().await;
        if let Err(e) = cm.run().await {
            eprintln!("Connection manager error: {}", e);
        }
    });

    (state, tx, socket, connection_manager)
}

#[tokio::test]
async fn test_game_state_transitions() {
    let (mut state, _tx, _socket, _cm) = setup_test_environment().await;

    assert_eq!(
        state.get_state(),
        GameState::WaitingForPlayers,
        "Initial state should be WaitingForPlayers"
    );

    // Add players
    state
        .add_player(
            PlayerId::Player1,
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap(),
        )
        .await
        .unwrap();
    state
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
    let (mut state, _tx, _socket, _cm) = setup_test_environment().await;

    // Add players and start game
    state
        .add_player(
            PlayerId::Player1,
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap(),
        )
        .await
        .unwrap();
    state
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

    // Simulate a score
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

async fn receive_packet_with_timeout(
    socket: &UdpSocket,
    duration: Duration,
) -> Option<ServerMessage> {
    timeout(duration, async {
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf).await {
            Ok((size, _)) => bincode::deserialize::<NetworkPacket<ServerMessage>>(&buf[..size])
                .ok()
                .map(|packet| packet.get_payload().clone()),
            Err(_) => None,
        }
    })
    .await
    .unwrap_or(None)
}

#[tokio::test]
async fn test_game_start_and_state_update() {
    let test_timeout = Duration::from_secs(10);
    let result = timeout(test_timeout, async {
        let (mut state, _tx, _server_socket, connection_manager) = setup_test_environment().await;

        // Create client sockets
        let client1_socket = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());
        let client2_socket = Arc::new(UdpSocket::bind("127.0.0.1:0").await.unwrap());

        // Add players
        state
            .add_player(PlayerId::Player1, client1_socket.local_addr().unwrap())
            .await
            .unwrap();
        state
            .add_player(PlayerId::Player2, client2_socket.local_addr().unwrap())
            .await
            .unwrap();

        // Create a channel to monitor outgoing messages from the server
        let (monitor_tx, mut monitor_rx) = mpsc::channel(100);
        let monitor_tx = Arc::new(Mutex::new(monitor_tx));

        // Set up the message monitor in the ConnectionManager
        {
            let mut cm = connection_manager.lock().await;
            cm.set_message_monitor(Arc::clone(&monitor_tx));
        }

        // Start the game
        state.start_game().unwrap();

        // Update state to trigger broadcast
        state.update().await.unwrap();

        // Check for messages with a timeout
        let messages_received = Arc::new(AtomicUsize::new(0));
        let messages_received_clone = Arc::clone(&messages_received);

        let message_handler = tokio::spawn(async move {
            let mut game_started_received = false;
            let mut position_update_received = false;
            let mut score_update_received = false;

            let start_time = std::time::Instant::now();
            let handler_timeout = Duration::from_secs(5);

            while start_time.elapsed() < handler_timeout
                && (!game_started_received || !position_update_received || !score_update_received)
            {
                tokio::select! {
                    Some(message) = receive_packet_with_timeout(&client1_socket, Duration::from_millis(100)) => {
                        match message {
                            ServerMessage::GameStateChange(GameState::GameStarted) => {
                                game_started_received = true;
                                messages_received_clone.fetch_add(1, Ordering::SeqCst);
                            }
                            ServerMessage::PositionUpdate(_) => {
                                position_update_received = true;
                                messages_received_clone.fetch_add(1, Ordering::SeqCst);
                            }
                            ServerMessage::ScoreUpdate(_) => {
                                score_update_received = true;
                                messages_received_clone.fetch_add(1, Ordering::SeqCst);
                            }
                            _ => {}
                        }
                    }
                    Some((packet, addr)) = monitor_rx.recv() => {
                        println!("Server sent message to {}: {:?}", addr, packet.get_payload());
                    }
                    else => {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                }
            }

            (game_started_received, position_update_received, score_update_received)
        });

        let (game_started, position_update, score_update) = message_handler.await.unwrap();

        assert!(
            game_started,
            "GameStateChange message for game start was not received"
        );
        assert!(
            position_update,
            "PositionUpdate message was not received"
        );
        assert!(
            score_update,
            "ScoreUpdate message was not received"
        );

        assert_eq!(
            messages_received.load(Ordering::SeqCst),
            3,
            "Did not receive all expected messages"
        );

        Ok::<_, Box<dyn std::error::Error>>(())
    })
    .await;

    match result {
        Ok(Ok(_)) => println!("Test completed successfully"),
        Ok(Err(e)) => panic!("Test failed: {:?}", e),
        Err(_) => panic!("Test timed out after {:?}", test_timeout),
    }
}
