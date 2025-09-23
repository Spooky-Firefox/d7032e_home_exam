# Copilot Instructions for Home Exam Codebase

## Overview
This codebase contains two main components:
- **VajbCruncher Java Game Implementation**: Located in `VajbCruncher/`, simulates a Rivals for Catan-like card game with local, bot, and online play.
- **Rust Starter Project**: Located in `src/`, currently a placeholder for future Rust code.

## Architecture & Data Flow
- The game logic is centralized in `Server.java`, which manages player setup, game loop, and communication.
- Players are represented by `Player.java` (local/bot) and `OnlinePlayer.java` (networked). Both share core logic, with `OnlinePlayer` adding socket-based communication.
- Cards and game state are modeled in `Card.java` and loaded from `cards.json` using GSON (see `gson.jar`).
- Communication between server and clients uses custom string messages over Java sockets. Prompts are sent as strings prefixed with `PROMPT:`.
- Game state (resources, cards, flags) is managed with public fields for exam transparency and quick prototyping.

## Developer Workflows
- **Build/Run Java**: Compile and run from `VajbCruncher/`:
  - `javac -cp gson.jar *.java`
  - `java -cp .:gson.jar Server [bot|online]`
- **Rust**: Use Cargo for builds/tests in the root directory:
  - `cargo build`
  - `cargo run`
- **Cards Data**: Card definitions are in `cards.json`. Update this file to change available cards.
- **Documentation**: Exam instructions and requirements are in `task_documentation/`.

## Project-Specific Patterns & Conventions
- **Intentionally Poor Design**: The Java code is deliberately designed with poor practices (e.g., public fields, lack of encapsulation, magic numbers) to serve as a baseline for the exam. The goal is for you to rewrite a well-designed version. Public fields are **not** for exam clarity or automatic testing.
- **Magic Numbers**: Player indices (0, 1) are used directly; consider using named constants for clarity.
- **String-Based Protocol**: All network communication is via serialized strings, not structured objects.
- **Quick & Dirty**: Code prioritizes exam speed and transparency over production robustness.
- **Card Loading**: Use `Card.loadBasicCards("cards.json")` to initialize card stacks.

## Integration Points
- **GSON**: Used for JSON parsing (`gson.jar`).
- **Socket Communication**: Online play uses Java sockets; see `OnlinePlayer.java` and `Server.java` for setup.
- **Exam Documentation**: Refer to `task_documentation/` for requirements and game rules.

## Key Files & Directories
- `VajbCruncher/Server.java`: Main game loop and server logic
- `VajbCruncher/Player.java`, `OnlinePlayer.java`: Player logic
- `VajbCruncher/Card.java`, `cards.json`: Card definitions and data
- `VajbCruncher/gson.jar`: JSON parsing library
- `src/main.rs`: Rust starter (currently trivial)
- `task_documentation/`: Exam instructions and game rules

## Example: Adding a New Card
1. Edit `cards.json` to add card details.
2. Ensure `Card.java` fields match new card properties.
3. Rebuild and restart the server.

---
For unclear or missing conventions, please provide feedback or specify areas needing more detail.
