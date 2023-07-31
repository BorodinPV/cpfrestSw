extern crate serde;


use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::net::SocketAddr;

use clap::Parser;
use config_file::FromConfigFile;
use hyper::{Body, Method, Request, Response, StatusCode};
use hyper::body::Buf;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use rustc_serialize::json::Json;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::utility::{utils, work_with_files};

mod utility;
mod configs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = configs::settings::Server::from_config_file("./config/properties.toml").unwrap();
    println!("{:?}", config.urls_set);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(handler)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => Ok(Response::new(Body::from("I'm fine."))),
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn jobber() {
    let mut map: HashMap<String, &Json> = HashMap::new();
    let response = utils::get_info_for_service(String::from("http://localhost:8383/s3client/v3/api-docs"));

    let component = utils::get_json_component("paths".parse().unwrap(), &response);
    map.insert("service".to_string(), component);
    println!("Значение: {:?}", map.get("service"));

    let file = work_with_files::create_file("s3client".to_string());
    work_with_files::write_file(file, "s3client".to_string(), component);

    println!("data written to file");
}