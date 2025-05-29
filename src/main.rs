use anyhow::{anyhow, Context};
use warp::Filter;
use std::collections::HashMap;
use std::env;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let port: u16 = env::var("CORS_PORT")
        .unwrap_or("9000".to_string())
        .parse()
        .expect("Invalid PORT value provided");

    let whitelist: Vec<_> = env::var("WHITELIST")
        .unwrap_or_default()
        .split(",")
        .filter(|s| !s.is_empty() )
        .map(|s| s.to_string())
        .collect();

    let cors = env::var("CORS").is_ok();

    let client = reqwest::Client::new();

    if whitelist.len() == 0 {
        info!("All domains whitelisted")
    } else {
        info!("Whitelisted domains: {:?}", whitelist)
    }

    let proxy = warp::path::end()
        .and(warp::filters::query::query::<HashMap<String, String>>())
        .and(warp::filters::method::method())
        .and(warp::filters::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then({
            move |queries, method, headers, body | {
                do_fetch(client.clone(), whitelist.clone(), cors.clone(), queries, method, body, headers)
            }
        });

    warp::serve(proxy).run(([0, 0, 0, 0], port)).await;
}

async fn fetch(
    client: reqwest::Client, 
    whitelist: Vec<String>,
    cors: bool,
    queries: HashMap<String,String>, 
    method: warp::http::Method,
    body: bytes::Bytes,
    headers: warp::http::HeaderMap
) 
    -> anyhow::Result<impl warp::Reply>{

    let url = reqwest::Url::parse(queries.get("url").context("missing url query")?)?;
    
    let url_domain = url.domain().context("no domain on given origin")?;

    if whitelist.len() > 0 && whitelist.iter().all(|whitelisted_domain| url_domain != whitelisted_domain) {
        return anyhow::Result::Err(anyhow!("url did not match any domains on whitelist"));
    }

    let mut request = client.request(method.clone(), url);

    for (key, value) in headers.iter() {
        if key != "host" && key != "origin" && key != "referer" {
            request = request.header(key, value);
        }
    }

    if matches!(method, reqwest::Method::POST | reqwest::Method::PUT | reqwest::Method::PATCH) {
        request = request.body(body);
    }

    info!("{request:?}");
    let response = request.send().await?;

    let status = response.status();
    let response_headers = response.headers().clone();
    let body = response.bytes().await.unwrap_or_default();

    let mut reply = warp::http::Response::builder().status(status);
    for (key, value) in response_headers.iter() {
        reply = reply.header(key, value);
    }

    if cors {
        reply = reply.header("Access-Control-Allow-Origin", "*");
    }

    return Ok(reply.body(body)?)
}

// need to do this bc rust doesn't support unnamed closures ?
async fn do_fetch(
    client: reqwest::Client, 
    whitelist: Vec<String>,
    cors: bool,
    queries: HashMap<String,String>, 
    method: warp::http::Method,
    body: bytes::Bytes,
    headers: warp::http::HeaderMap
) -> Result<impl warp::Reply, warp::reject::Rejection>{
    fetch(client, whitelist, cors, queries, method, body, headers).await.map_err(|err| {
        info!("request failed: {}", err.to_string());
        warp::reject()
    })
}