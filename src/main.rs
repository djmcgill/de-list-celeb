use egg_mode;
use futures::stream::Stream;
use std::env;
use chrono::Utc;
use chrono::offset::TimeZone;
use std::thread;
use std::time;

use tokio_core::reactor::Core;

fn main() -> Result<(), Box<std::error::Error>> {
    let consumer_key = env::var("CONSUMER_KEY")?;
    let consumer_secret = env::var("CONSUMER_SECRET")?;
    let access_token_key = env::var("ACCESS_TOKEN_KEY")?;
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET")?;

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_token_key, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let mut runtime = Core::new()?;
    let handle = runtime.handle();

    let user = runtime.run(egg_mode::verify_tokens(&token, &handle))?;
    let list_iter = egg_mode::list::memberships(user.id, &token, &handle);
    let list_vec = runtime.run(list_iter.collect())?;
    println!("{:?}", list_vec);

    for list in list_vec {
        let list_creator_id = list.user.id;
        if list_creator_id != user.id {
            let response = runtime.run(egg_mode::user::block(list_creator_id, &token, &handle))?;
            if response.rate_limit_remaining <= 0 {
                println!("RATE LIMITED!");
                let time_to_resume = Utc.timestamp_millis(response.rate_limit_reset as i64 * 1000);
                println!("GOING TO RESUME AT {}", time_to_resume);
                let now = Utc::now().timestamp_millis();
                let sleep_millis = response.rate_limit_reset as i64 * 1000 - now;
                println!("SLEEPING FOR {:?} MILLIS", sleep_millis);
                thread::sleep(time::Duration::from_millis(sleep_millis as u64));
            }
        }
    }

    Ok(())
}
