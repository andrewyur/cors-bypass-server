use anyhow::Context;
use warp::Filter;
use std::collections::HashMap;
use std::env;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Invalid PORT value provided");

    let client = reqwest::Client::new();

    let proxy = warp::path::end()
        .and(warp::filters::query::query::<HashMap<String, String>>())
        .and(warp::filters::method::method())
        .and(warp::filters::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then({
            move |queries, method, headers, body | {
                do_fetch(client.clone(), queries, method, body, headers)
            }
        });

    warp::serve(proxy).run(([0, 0, 0, 0], port)).await;
}

async fn fetch(
    client: reqwest::Client, 
    queries: HashMap<String,String>, 
    method: warp::http::Method,
    body: bytes::Bytes,
    headers: warp::http::HeaderMap
) 
    -> anyhow::Result<impl warp::Reply>{

    let url = queries.get("url").context("missing url query")?;

    let mut request = client.request(method.clone(), url);

    // omit headers involved with CORS
    for (key, value) in headers.iter() {
        if key != "host" && key != "origin" && key != "referer" {
            request = request.header(key, value);
        }
    }

    // Add body if appropriate
    if matches!(method, reqwest::Method::POST | reqwest::Method::PUT | reqwest::Method::PATCH) {
        request = request.body(body);
    }

    let response = request.send().await?;

    let status = response.status();
    let response_headers = response.headers().clone();
    let body = response.bytes().await.unwrap_or_default();

    let mut reply = warp::http::Response::builder().status(status);
    for (key, value) in response_headers.iter() {
        reply = reply.header(key, value);
    }

    reply = reply.header("Access-Control-Allow-Origin", "*");

    return Ok(reply.body(body)?)
}

// need to do this bc rust doesn't support unnamed closures ?
async fn do_fetch(
    client: reqwest::Client, 
    queries: HashMap<String,String>, 
    method: warp::http::Method,
    body: bytes::Bytes,
    headers: warp::http::HeaderMap
) -> Result<impl warp::Reply, warp::reject::Rejection>{
    fetch(client.clone(), queries, method, body, headers).await.map_err(|err| {
        warn!("error: {}", err.to_string());
        warp::reject()
    })
}