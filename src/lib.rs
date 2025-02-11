mod ezkv;
mod method;
mod newlib;

use crate::method::HttpMethod;

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

fn make_client(cookies: Arc<Jar>) -> Client {
    let client_builder: ClientBuilder = Client::builder();
    let client: Client = client_builder
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

async fn _http_request(
    method: HttpMethod,
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    dbg!(format!("method: {:?}", method));
    dbg!(format!("url: {}", url_str));
    dbg!(format!("cookie: {}", cookie_str));
    dbg!(format!("header: {:?}", &headers));
    dbg!(format!("data: {}", data_str));

    let url = make_url(url_str);
    let cookies = make_cookie(cookie_str, &url);

    let client: Client = make_client(cookies);

    let request_builder: RequestBuilder = match method {
        HttpMethod::GET => client.get(url),
        HttpMethod::POST => client.post(url),
        HttpMethod::PUT => client.put(url),
        HttpMethod::DELETE => client.delete(url),
    };

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

    res_str
}

pub async fn http_get(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    _http_request(HttpMethod::GET, url_str, cookie_str, headers, data_str).await
}

pub async fn http_post(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    _http_request(HttpMethod::POST, url_str, cookie_str, headers, data_str).await
}

pub async fn http_put(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    _http_request(HttpMethod::PUT, url_str, cookie_str, headers, data_str).await
}

pub async fn http_delete(
    url_str: &str,
    cookie_str: &str,
    headers: HashMap<&str, &str>,
    data_str: &str,
) -> String {
    _http_request(HttpMethod::DELETE, url_str, cookie_str, headers, data_str).await
}

#[cfg(test)]
mod tests {
    use super::*;

    static BASE_URL: &str = "https://httpbin.org";

    #[test]
    fn test_http_get() {
        let url = format!("{BASE_URL}/get");
        let cookie = "";
        let headers = HashMap::new();
        let data = "";

        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(async { http_get(url.as_str(), cookie, headers, data).await });

        println!("{}", response);
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&response).unwrap();

        assert!(data.contains_key("args"));
        assert_eq!(0, data["args"].as_object().unwrap().len());

        assert!(data.contains_key("headers"));
        assert_eq!(4, data["headers"].as_object().unwrap().len());
    }

    #[test]
    fn test_http_post() {
        let url = format!("{BASE_URL}/post");
        let cookie = "";
        let headers = HashMap::new();
        let data = "";

        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(async { http_post(url.as_str(), cookie, headers, data).await });

        println!("{}", response);
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&response).unwrap();

        assert!(data.contains_key("args"));
        assert_eq!(0, data["args"].as_object().unwrap().len());

        assert!(data.contains_key("headers"));
        assert_eq!(4, data["headers"].as_object().unwrap().len());
    }

    #[test]
    fn test_http_put() {
        let url = format!("{BASE_URL}/put");
        let cookie = "";
        let headers = HashMap::new();
        let data = "";

        let rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(async { http_put(url.as_str(), cookie, headers, data).await });

        println!("{}", response);
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&response).unwrap();

        assert!(data.contains_key("args"));
        assert_eq!(0, data["args"].as_object().unwrap().len());

        assert!(data.contains_key("headers"));
        assert_eq!(4, data["headers"].as_object().unwrap().len());
    }

    #[test]
    fn test_http_delete() {
        let url = format!("{BASE_URL}/delete");
        let cookie = "";
        let headers = HashMap::new();
        let data = "";

        let rt = tokio::runtime::Runtime::new().unwrap();
        let response =
            rt.block_on(async { http_delete(url.as_str(), cookie, headers, data).await });

        println!("{}", response);
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&response).unwrap();

        assert!(data.contains_key("args"));
        assert_eq!(0, data["args"].as_object().unwrap().len());

        assert!(data.contains_key("headers"));
        assert_eq!(4, data["headers"].as_object().unwrap().len());
    }
}
