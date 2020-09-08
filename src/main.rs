use std::env;
use std::thread;
use std::time::Duration;
use dotenv::dotenv;
use mobc::Pool;
use mobc_redis::redis;
use mobc_redis::RedisConnectionManager;
use mobc_redis::{Connection};
use serde::{Deserialize, Serialize};
use std::thread::sleep;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // redis
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");
    let client = redis::Client::open(redis_url).unwrap();
    let manager = RedisConnectionManager::new(client);
    let redis_pool = Pool::builder().max_open(10).build(manager);


    let args: Vec<String> = env::args().collect();
    let len = args.len();

    if len >= 2 {
        let _action = &args[1];
        if _action == "lpop" {
            _lpop(redis_pool).await;
        }
        if _action == "test" {
            _test_action().await;
        }
    }

    println!("success");
    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct Pushdata {
    id: i64,
    name: String,
}

async fn _lpop(_redis_pool: mobc::Pool<RedisConnectionManager>) {
    loop {
        let mut conn = _redis_pool.get().await.unwrap();
        let s = redis::cmd("LPOP")
            .arg("list")
            .query_async(&mut conn as &mut Connection)
            .await
            .unwrap_or("".to_string());

        println!("s: {:#?}", s);

        sleep(Duration::new(1, 0));
    }
}


async fn _test_action() {
    loop {
        let _ret = thread::spawn(|| {
            for i in 0..2 {
                println!("spawned thread print {}", i);
                //thread::sleep(Duration::from_millis(10));
            }
        })
            .join().unwrap();
        sleep(Duration::new(1, 0));
    }
    //println!("_ret: {:#?}", _ret);
}

