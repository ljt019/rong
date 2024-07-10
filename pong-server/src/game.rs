// A game has 2 players. Each player has a unique ID, a socket address, and a x and y position
// A game has 1 ball. The ball has a x and y position.

// The game has 3 states: WaitingForPlayers, WaitingForPlayer2, and GameStarted
// The game starts in the WaitingForPlayers state. When a player sends a "CONNECT" message, the game transitions to the WaitingForPlayer2 state. When a second player sends a "CONNECT" message, the game transitions to the GameStarted state.
