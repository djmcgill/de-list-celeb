use egg_mode;
use std::env;
fn main() -> Result<(), Box<std::error::Error>> {
    let consumer_key = env::var("CONSUMER_KEY")?;
    let consumer_secret = env::var("CONSUMER_SECRET")?;
    println!("Hello, world!");
    Ok(())
}
