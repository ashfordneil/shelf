Shelf
==============

A collaborative card based document editing tool.
![Screenshot](https://lh5.googleusercontent.com/iJ5wXINY1P6Cv1XG2THWqZ8Oa3HF59MP-WtBvBUY-1S61TRfTij_VrvA0VrpBes-vzzZjUvvY0RAr2VprCmU45KBct_eLBJs3zN-MEuOsuCqVrDY6_QujK3GPT6dYjUoT_9Lrs0EwNk)

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
