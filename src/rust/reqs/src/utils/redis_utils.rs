extern crate redis;
extern crate futures;
extern crate tokio;

use futures::Future;
use redis::Client;
use chrono::prelude::*;

pub fn get_redis_connection() -> Client{
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();    
    client.get_connection.unwrap()
}

pub struct CachedGlobalReqService {
    service: GlobalReqService,
    stale_at: Datetime<Utc>
}

pub fn get_cached_global_service(client: Client, )