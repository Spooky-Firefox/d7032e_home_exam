# Todo

## Server

- [ ] Add a server
- [ ] Make dice rely on a connection to a server to ensure clients are in sync

## Initialization

- [ ] Move initialization code to its own folder
- [ ] Make `Game` contain a Vec of functions/objects which initialize stuff

## Terminal TUI

- [ ] Render the boards in the terminal UI

## ECS

- [ ] Add components such as owner
- [ ] Add both dice
- [ ] Add Player token to denote the active player
- [ ] Add trade advantage and hero tokens

## Util

- [ ] Add functions that aid in card placement

## Common

- [ ] Consider if there needs to be a better player decision API

## Phases

- [ ] Phase to only move player token
- [ ] Phase to roll dice
- [ ] Resolve event dice (use player token to see active player)
- [ ] Production dice phase
- [ ] Cards on board phase (example)
- [ ] Action phase
  - [ ] Placing cards
  - [ ] Action cards
  - [ ] Center cards (roads, settlements, regions, etc.)
  - [ ] Trading resources
- [ ] Pick cards
- [ ] Exchange cards
- [ ] Check/move Strength advantage and Trade advantage markers
- [ ] VP points check and declare victory

## Concerns

- [ ] Are the queries deterministic? Can one rely on the query result order to avoid desync issues?
