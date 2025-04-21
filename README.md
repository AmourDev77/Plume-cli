# Current state
This cli is not meant to be the final version of the applciation, but more a testing tool hosted here for development purpose.  

In it's current state, it create a connexion to a relay server and allow user to send / receive messages from all others connected on the relay.  

# Installation
## Requirements
- cargo

## Steps (for development purpose)
1. Clone this repository
2. Copy the file `.env-template` to `.env` and complete each fields
3. Run `cargo run`

# Incoming changes
* [ ] ED25519 & x25519 key Registeration 
* [ ] URL Selection
* [ ] Connection / Leave commands

# Useful informations
To get full documentation on the project, please run the command `cargo doc --opoen`.
