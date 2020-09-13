use mobc_redis::Connection;
use mobc_redis::RedisConnectionManager;
use mobc_redis::redis;
/*use std::time::Duration;
use std::thread::sleep;*/

pub fn function1(num: i32) -> i32 {
    num + 100
}

pub async fn lpop(_redis_pool: mobc::Pool<RedisConnectionManager>) -> bool {
    let mut conn = _redis_pool.get().await.unwrap();
    let s = redis::cmd("LPOP")
        .arg("list")
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or("".to_string());

    println!("s: {:#?}", s);
    true
    //sleep(Duration::new(1, 0));
}

pub async fn aaaa() -> bool {
    tokio::time::delay_for(tokio::time::Duration::from_secs(2)).await;
    println!("s: {:#?}", "f3");
    true
}