# CaptureTheFlag

CaptureTheFlag is a multiplayer arena game implemented as a Rust desktop application. The project follows the rules and communication model described in [PRFC-VERSION-3.md](PRFC-VERSION-3.md), using a binary client/server protocol. The goal is to provide a complete game experience in which players compete to capture the flag, return it outside the central circle, and win before the opposing players do.

## Project Overview

This project is structured as a small but complete real-time game architecture:

- A launcher entry point starts the application and selects whether the machine will host a server or join a client session.
- A server-side layer maintains the authoritative game state, processes player inputs, advances the world simulation, and validates every gameplay action.
- A client-side layer connects to the server, sends input, receives state updates, and renders the game world locally.
- A protocol layer defines the binary message format so that clients and servers can exchange game state consistently.
- A GUI layer handles rendering, overlays, camera behavior, and user interface feedback.

The result is a modular design where gameplay rules, networking, and presentation are separated into distinct layers.

## Build and Run

From the project root, build and run the application with:

```bash
cargo run
```

The project uses Rust and the following main dependencies:

- macroquad for the game window and rendering
- rand for gameplay-related randomness

## How to Play

1. Launch the application and choose whether to host a server or join an existing match.
2. If you host a server, wait for other players to connect to your session.
3. Once the match starts, move around the arena and compete to reach the flag.
4. Pick up the flag, leave the central circle, and return outside it to win the round.
5. If you are joining a match, follow the server's state and play within the rules defined by the protocol.

## Project Structure

The source tree is organized around clear responsibilities:

- src/app: application mode handling and the main runner
- src/server: server logic, game state updates, and networking
- src/client: client connection logic and client-side update flow
- src/protocol: shared protocol structures and message serialization
- src/gui: rendering, camera management, and overlays
- src/config: configuration values for the game

## Architecture

### 1. Main Application

The application starts from the main entry point and creates a single runtime object that can switch between launcher, server-host, and client-join modes. This makes the project behave like a unified client application with multiple execution modes rather than a collection of unrelated programs.

### 2. Server-oriented game model

The server is the authority of the match. It owns the official map state, player positions, flag status, and win conditions. Each update cycle advances the simulation, applies movement rules, checks interactions, and broadcasts the latest state to connected clients. This design keeps the game logic centralized and makes the rules easier to reason about.

### 3. Client-side interaction model

Clients are responsible for presenting the game to the user and sending input intentions to the server. They do not decide the outcome of gameplay on their own; instead, they follow the server's authoritative state. This separation helps keep the experience consistent across different machines and implementations.

### 4. Binary protocol foundation

The protocol layer is based on [PRFC-VERSION-3.md](PRFC-VERSION-3.md). That specification defines the game rules, the server/client responsibilities, and the binary message layout used by the project. The implementation uses that protocol as the contract for communication between all parties.

### 5. Rendering and UI

The graphical layer is responsible for drawing the world, players, UI overlays, and logs. The camera and rendering modules are designed to reflect the game state that arrives from the server, creating a smooth view of the match as it evolves.

## Notes

CaptureTheFlag is designed as a teaching-oriented multiplayer game project that emphasizes architecture, networking, and protocol-driven development. Its main purpose is not only to play the game, but also to demonstrate how a real-time client/server system can be organized around a shared specification.
