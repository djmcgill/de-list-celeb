# De-List Celeb

Are you being put on twitter lists you don't want to be? Make yourself de-list by blocking the creator of every list you're on, removing yourself from them all.

## Run
1. Copy `dotenv_sample` to `.env`, fill in the values from https://developer.twitter.com OR set vars manually with `export`.
2. Currently just `env $(cat .env | xargs) cargo run`

## TODO:
1. Actually implement
2. Sort out permissions
    - Better explanation, link to twitter docs
3. Sort out deployment
    - Binary?
    - Browser plugin?
    - SAAS?
