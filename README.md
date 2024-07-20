# Rong

This project is a networked implementation of the classic Pong game written in Rust. It consists of a server application that manages the game state and client applications that handle rendering and user input.

## For Players

### Quick Start

1. Download the latest client release for your operating system from the [Releases](https://github.com/your-username/networked-pong/releases) page.
2. Extract the downloaded archive.
3. Run the `pong-client` executable.

### How to Play

1. When you start the game, it will automatically connect to the public game server.
2. Once another player connects, the game will start automatically.
3. Use the left and right arrow keys to move your paddle.
4. Try to hit the ball past your opponent's paddle to score points.
5. The game continues until you close the client window.

### System Requirements

- Windows 10/11, macOS 10.15+, or Linux (Ubuntu 20.04+)
- OpenGL 3.3+ compatible graphics card

If you encounter any issues, please check the [Troubleshooting](#troubleshooting) section below or open an issue on GitHub.

## For Developers

### Features

- Multiplayer Pong game over a network
- Server-side game logic
- Client-side rendering using Macroquad
- UDP-based communication between server and clients

### Project Structure

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

### Prerequisites

- Rust programming language (latest stable version)
- Cargo (Rust's package manager)

### Setup

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

### Running the Game (Development)

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
