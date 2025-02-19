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
use serde_json;
use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;
use std::time::Duration;

pub enum Header {
    text(String),
    json(serde_json::Value),
}

pub enum Body {
    text(String),
    json(serde_json::Value),
}

pub struct Chitose {
    cookies: Arc<Jar>,
}

impl Chitose {
    pub fn set_cookies(&mut self, cookie_str: impl Into<String>, url: &Url) -> &Self {
        let cookies = Arc::new(Jar::default());

        Into::<String>::into(cookie_str)
            .split("; ")
            .for_each(|e| cookies.add_cookie_str(e, url));

        self.cookies = cookies;

        self
    }

    pub fn get_cookie(&mut self) -> Arc<Jar> {
        self.cookies.clone()
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

        let onetime_headers: HeaderMap = Self::make_default_header(headers);
        let mut response = Self::receive_response(request_builder, onetime_headers, data_str).await;

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
}
