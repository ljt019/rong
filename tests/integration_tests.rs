use bincode;
use rong_server::network::GameServer;
use rong_shared::model::{ClientMessage, NetworkPacket, PlayerId, ServerMessage};
use std::io;
use tokio::net::UdpSocket;
use tokio::time::Duration;

const SERVER_ADDR: &str = "127.0.0.1:2906";

#[tokio::test]
async fn test_basic_connection() -> io::Result<()> {
    println!("Starting basic connection test");

    // Create a mock client
    let client_socket = UdpSocket::bind("0.0.0.0:0").await?;
    println!("Client socket bound to {}", client_socket.local_addr()?);

    // Send connect message
    let connect_message = NetworkPacket::new(1, 0, ClientMessage::Connect());
    let serialized = bincode::serialize(&connect_message).expect("Failed to serialize message");
    println!("Sending bytes: {:?}", serialized);
    client_socket.send_to(&serialized, SERVER_ADDR).await?;
    println!("Connect message sent");

    // Wait for PlayerJoined response
    let mut buf = [0; 1024];
    let result =
        tokio::time::timeout(Duration::from_secs(5), client_socket.recv_from(&mut buf)).await;

    match result {
        Ok(Ok((size, addr))) => {
            println!("Received {} bytes from {}", size, addr);
            let response: NetworkPacket<ServerMessage> =
                bincode::deserialize(&buf[..size]).expect("Failed to deserialize response");

            match response.get_payload() {
                ServerMessage::PlayerJoined(player_id) => {
                    assert!(
                        *player_id == PlayerId::Player1 || *player_id == PlayerId::Player2,
                        "Received invalid player ID"
                    );
                    println!("Received PlayerJoined with ID: {:?}", player_id);
                }
                _ => panic!("Expected PlayerJoined message, got {:?}", response),
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error receiving response: {}", e);
            return Err(e);
        }
        Err(_) => {
            eprintln!("Timeout waiting for server response");
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Server response timeout",
            ));
        }
    }

    println!("Basic connection test passed successfully");
    Ok(())
}
