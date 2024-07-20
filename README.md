# Rong

This project is a networked implementation of the classic Pong game written in exclusively Rust. It consists of a server application that manages the game state and client applications that handle rendering and user input.

## Features

- Multiplayer Pong game over a network
- Server-side game logic
- Client-side rendering using Macroquad
- UDP-based communication between server and clients

## Project Structure

The project is divided into two main components:

1. Server

   - `main.rs`: Entry point for the server application
   - `game.rs`: Implements the core game logic
   - `ball.rs`: Defines the ball behavior
   - `players.rs`: Manages player state and movement

2. Client
   - `main.rs`: Entry point for the client application
   - `game.rs`: Manages the game state and rendering
   - `ball.rs`: Defines the ball rendering
   - `opponent.rs`: Manages opponent rendering
   - `player.rs`: Handles player input and rendering
   - `server.rs`: Manages communication with the server

## Prerequisites

- Rust programming language (latest stable version)
- Cargo (Rust's package manager)

## Setup

1. Clone the repository:

   ```
   git clone https://github.com/your-username/networked-pong.git
   cd networked-pong
   ```

2. Build the server:

   ```
   cd server
   cargo build --release
   ```

3. Build the client:
   ```
   cd ../client
   cargo build --release
   ```

## Running the Game

1. Start the server:

   ```
   cd server
   cargo run --release
   ```

   The server will start listening on `0.0.0.0:2906`.

2. Start two client instances:
   ```
   cd client
   cargo run --release
   ```
   Run this command in two separate terminal windows to start two clients.

## How to Play

1. Once both clients are connected, the game will start automatically.
2. Use the left and right arrow keys to move your paddle.
3. Try to hit the ball past your opponent's paddle to score points.
4. The game continues until you close the client window.

## Network Protocol

The game uses a simple UDP-based protocol for communication between the server and clients. For more details, see the `game-protocol-messages.md` file in the project root.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The Macroquad library for providing easy-to-use game development tools for Rust.
- The Rust community for their excellent documentation and support.
