Scaling Engine
==============

A collaborative card based document editing tool.

# Front-end
Run with `yarn start`
Build for release with `yarn build`

# Back-end
Run with `cargo run`
Build for release with `cargo build --release`

# Testing
Navigate to localhost:8080/

# API
`/board` => POST
returns a UUID representing a newly created empty board

`/board/:id` => GET
takes a UUID representing a board, returns an array of UUIDs representing tiles in that board
