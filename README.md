# De-List Celeb

Are you being put on twitter lists you don't want to be? Make yourself de-list by blocking the creator of every list you're on, removing yourself from them all.

## Run
1. Install Rust via rustup.rs
2. Copy `dotenv_sample` to `.env`, fill in the values from https://developer.twitter.com OR set vars manually with `export`.
3. Currently just `env $(cat .env | xargs) cargo run` from bash

## TODO:
1. Sort out permissions
    - Better explanation, link to twitter docs
2. Sort out deployment
    - Binary?
    - Browser plugin?
    - Website?

Note can only do 300 blocks/hr. Maybe bypass this by asking for another user auth token?
