#[macro_use]
extern crate redis_async;

use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use redis_async::resp::RespValue;
use futures::future::join_all;
mod data;

async fn heartbeat(
    info: web::Json<data::Server>,
    redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
    let info = info.into_inner();
    let json = serde_json::to_string(&info)?;
    let key = format!("{}:{}", info.ip, info.port);
    println!("----> Adding server, key: {}, value: {}", key, json);
    
    let res = redis.send(Command(resp_array!["SET", key, json, "EX", "15"]))
        .await?;

    match res {
        Ok(RespValue::Nil) | Ok(RespValue::SimpleString(_)) => {
            Ok(HttpResponse::Ok().finish())
        }
        _ => {
            println!("---->{:?}", res);
            Ok(HttpResponse::InternalServerError().body("Failed to update"))
        }
    }
}

async fn get_stuff(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    let res = redis.send(Command(resp_array!["KEYS", "*"]))
        .await?;

    

    match res {
        Ok(RespValue::Array(x)) => {
            println!("res: {:?}", x.clone());
            let keys = x.into_iter().filter_map(|value| {
                match value {
                    RespValue::BulkString(key) => {Some(key)}
                    _ => {
                        println!("Key did not match type, value: {:?}", value);
                        None
                    }
                }
            });

            
            let reqs = keys.map(|key| {
                redis.send(Command(resp_array!["GET", key]))
            });


            let ress: Vec<data::Server> = join_all(reqs)
                    .await
                    .into_iter()
                    .filter_map(|item| {
                        match item {
                            Ok(Ok(RespValue::BulkString(value))) => {String::from_utf8(value).ok()}
                            _ => {
                                println!("Value did not match type, value: {:?}", item);
                                None
                            }
                        }
                    })
                    .filter_map(|server_str| {
                        serde_json::from_str::<data::Server>(&server_str).ok()
                    })
                    .collect();

            Ok(HttpResponse::Ok().json(ress))
        }
        _ => {
            println!("---->{:?}", res);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    env_logger::init();

    HttpServer::new(|| {
        let redis_addr = RedisActor::start("redis:6379");

        App::new()
            .data(redis_addr)
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::post().to(heartbeat))
                    .route(web::get().to(get_stuff)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}