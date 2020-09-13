mod commands;

use crate::commands::*;
use dotenv::dotenv;
use mobc::Pool;
use mobc_redis::redis;
//use mobc_redis::Connection;
use mobc_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use futures;
/*use std::cell::RefCell;
use std::rc::Rc;*/

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
            let f1 = bll::lpop(redis_pool.clone());
            let f2 = bll::lpop(redis_pool.clone());
            let f3 = bll::aaaa();
            // 使用await则会顺序执行，使用join则会并发执行f1和f2
            // f1.await;
            // f2.await;
            let (r1 , r2, r3) = futures::join!(f1, f2, f3);
            println!("r1= {} , r2 = {}, r3 = {} ", r1 , r2, r3 );
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

async fn _sleep(_redis_pool: mobc::Pool<RedisConnectionManager>) {
    let cpus = num_cpus::get();
    loop {
        let _ret = thread::spawn(move || {
            for i in 0..cpus {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_secs(1));
            }
        })
            .join()
            .unwrap();

        sleep(Duration::new(1, 0));
    }
}

async fn _test_action() {
    let lock_sub = Arc::new(Mutex::new(0));
    let lock_sub1 = Arc::new(Mutex::new(0));
    let _ret = thread::spawn(move || {
        for i in 0..8 {
            println!("spawned thread print {}", i);

            let lock = Arc::clone(&lock_sub);
            let mut num = lock.lock().unwrap();
            *num += 2;

            let lock_v = Arc::clone(&lock_sub1);
            let mut v = lock_v.lock().unwrap();
            *v += bll::function1(100);
        }
        println!("Result  : {}", *lock_sub.lock().unwrap());
        println!("Result1  : {}", *lock_sub1.lock().unwrap());
    })
    .join()
    .unwrap();
}
