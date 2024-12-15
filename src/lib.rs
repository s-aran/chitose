use reqwest::cookie::Jar;
use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::RequestBuilder;
use reqwest::Response;
use reqwest::Url;
use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;
use std::time::Duration;

fn make_url(url_str: &str) -> Url {
    match Url::parse(url_str) {
        Ok(url) => url,
        Err(e) => panic!("URL parse error: {}", e),
    }
}

fn make_cookie(cookie_str: &str, url: &Url) -> Arc<Jar> {
    let cookies = Arc::new(Jar::default());

    cookie_str
        .split("; ")
        .for_each(|e| cookies.add_cookie_str(e, url));

    cookies
}

fn make_default_header(headers: HashMap<&str, &str>) -> HeaderMap {
    let mut default_headers: HeaderMap = HeaderMap::new();

    dbg!("header");
    for (k, v) in headers.iter() {
        dbg!(k);
        dbg!(v);
        default_headers.insert(
            HeaderName::try_from(k.to_owned()).unwrap(),
            v.parse().unwrap(),
        );
    }

    default_headers
}

fn make_client(default_headers: HeaderMap, cookies: Arc<Jar>) -> Client {
    let client_builder: ClientBuilder = Client::builder();
    let client: Client = client_builder
        // .default_headers(default_headers)
        .cookie_provider(cookies)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    client
}

async fn receive_response(
    request_builder: RequestBuilder,
    onetime_headers: HeaderMap,
    data_str: &str,
) -> Response {
    let response: Response = request_builder
        .headers(onetime_headers)
        .body(data_str.to_owned())
        // .query(&queries)
        .send()
        .await
        .unwrap();

    response
}

pub async fn http_get(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!("GET");
    dbg!(format!("url: {}", url_str));
    dbg!(format!("cookie: {}", cookie_str));
    dbg!(format!("header: {:?}", &headers));
    dbg!(format!("data: {}", data_str));

    let url = make_url(url_str);
    let cookies = make_cookie(cookie_str, &url);

    let default_headers: HeaderMap = HeaderMap::new();
    let client: Client = make_client(default_headers, cookies);

    let request_builder: RequestBuilder = client.get(url);

    let onetime_headers: HeaderMap = make_default_header(headers);
    let mut response = receive_response(request_builder, onetime_headers, data_str).await;

    let res_str = match response.headers().get(header::TRANSFER_ENCODING) {
        Some(v) if v == "chunked" => {
            let mut raw_res = Vec::new();
            while let Some(chunk) = response.chunk().await.unwrap() {
                chunk.to_vec().into_iter().for_each(|x| raw_res.push(x));
            }
            String::from_utf8(raw_res).unwrap()
        }
        _ => response.text().await.unwrap(),
    };

    println!("{}", res_str);

    res_str
}

pub async fn http_post(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!("POST");
    dbg!(format!("url: {}", url_str));
    dbg!(format!("cookie: {}", cookie_str));
    dbg!(format!("header: {:?}", &headers));
    dbg!(format!("data: {}", data_str));

    let url = make_url(url_str);
    let cookies = make_cookie(cookie_str, &url);

    let default_headers: HeaderMap = HeaderMap::new();
    let client: Client = make_client(default_headers, cookies);

    let request_builder: RequestBuilder = client.post(url);

    let onetime_headers: HeaderMap = make_default_header(headers);
    let mut response = receive_response(request_builder, onetime_headers, data_str).await;

    let res_str = match response.headers().get(header::TRANSFER_ENCODING) {
        Some(v) if v == "chunked" => {
            let mut raw_res = Vec::new();
            while let Some(chunk) = response.chunk().await.unwrap() {
                chunk.to_vec().into_iter().for_each(|x| raw_res.push(x));
            }
            String::from_utf8(raw_res).unwrap()
        }
        _ => response.text().await.unwrap(),
    };

    println!("{}", res_str);

    res_str
}

pub async fn http_put(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!("PUT");
    dbg!(format!("url: {}", url_str));
    dbg!(format!("cookie: {}", cookie_str));
    dbg!(format!("header: {:?}", &headers));
    dbg!(format!("data: {}", data_str));

    let url = make_url(url_str);
    let cookies = make_cookie(cookie_str, &url);

    let default_headers: HeaderMap = HeaderMap::new();
    let client: Client = make_client(default_headers, cookies);

    let request_builder: RequestBuilder = client.put(url);

    let onetime_headers: HeaderMap = make_default_header(headers);
    let mut response = receive_response(request_builder, onetime_headers, data_str).await;

    let res_str = match response.headers().get(header::TRANSFER_ENCODING) {
        Some(v) if v == "chunked" => {
            let mut raw_res = Vec::new();
            while let Some(chunk) = response.chunk().await.unwrap() {
                chunk.to_vec().into_iter().for_each(|x| raw_res.push(x));
            }
            String::from_utf8(raw_res).unwrap()
        }
        _ => response.text().await.unwrap(),
    };

    println!("{}", res_str);

    res_str
}

pub async fn http_delete(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!("DELETE");
    dbg!(format!("url: {}", url_str));
    dbg!(format!("cookie: {}", cookie_str));
    dbg!(format!("header: {:?}", &headers));
    dbg!(format!("data: {}", data_str));

    let url = make_url(url_str);
    let cookies = make_cookie(cookie_str, &url);

    let default_headers: HeaderMap = HeaderMap::new();
    let client: Client = make_client(default_headers, cookies);

    let request_builder: RequestBuilder = client.delete(url);

    let onetime_headers: HeaderMap = make_default_header(headers);
    let mut response = receive_response(request_builder, onetime_headers, data_str).await;

    let res_str = match response.headers().get(header::TRANSFER_ENCODING) {
        Some(v) if v == "chunked" => {
            let mut raw_res = Vec::new();
            while let Some(chunk) = response.chunk().await.unwrap() {
                chunk.to_vec().into_iter().for_each(|x| raw_res.push(x));
            }
            String::from_utf8(raw_res).unwrap()
        }
        _ => response.text().await.unwrap(),
    };

    println!("{}", res_str);

    res_str
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
