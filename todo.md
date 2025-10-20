# Todo

## Server / multiple players

- [ ] make a wrapper around decision strategy which directs the decision to remote/other player when running
- [ ] Add a server
- [ ] Make dice rely on a connection to a server to ensure clients are in sync

## Initialization

- [ ] Move initialization code to its own folder
- [ ] Initialize all card positioning
- [ ] Add resource to them
- [ ] move player marker and add trade advantage token ect.
- [ ] Make `Game` contain a Vec of functions/objects which initialize stuff

## Terminal TUI

- [ ] Render the boards in the terminal UI

## ECS

- [ ] Add components such as owner
- [ ] Add Resource components
  - [ ] Add resources to json
  - [ ] set starting resources
- [ ] Add box dyn player action?
  - this might not be a good idea, but how else would one implement dynamic behavior
- [ ] Add both dice
- [ ] Add Player token to denote the active player
- [ ] Add trade advantage and hero tokens

## Util

- [ ] Add functions that aid in card placement

## Common

- [ ] Consider if there needs to be a better player decision API
- [ ] Consider the implication of phase having a lock on world when we ask the decision api, is insertion &mut or not

## Phases

- [ ] make a looping phase which injects a player chose on the last one as to abort the loop or all ways

- [ ] Phase to only move player token
- [ ] Phase to roll dice
- [ ] Resolve event dice (use player token to see active player)
- [ ] Production dice phase
- [ ] Cards on board phase (example a card that doubles resources)
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
