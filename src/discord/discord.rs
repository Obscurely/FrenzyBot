use regex::Regex;
use reqwest;
use std::collections::HashMap;
use std::time;
use tokio::task;


pub struct Discord {
    token: String,
    client: reqwest::Client,
}

impl Discord {
    pub async fn from(token: String) -> Option<Discord> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(token.as_str()).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .https_only(true)
            .build()
            .unwrap();

        let check_token_res = client.get("https://discord.com/api/v9/users/@me").send().await.unwrap();
        if check_token_res.status() != reqwest::StatusCode::OK {
            // println!("{}", headers.get(reqwest::header::AUTHORIZATION).unwrap().to_str().unwrap());
            println!("Reached");
            return None;
        }

        Some(Discord {token, client})
    }

    pub async fn get_10_mess_in_dm(&self, dm_id: u64) -> String {
        let mess_res = self.client.get(format!("https://discord.com/api/v9/channels/{}/messages?limit=10", dm_id)).send().await.unwrap();
        mess_res.text().await.unwrap()
    }

    pub async fn send_mess_in_dm(&self, message: String, dm_id: u64) {
        let body = format!(
            "{{ \
            \"content\": \"{}\" \
        }}",
            message,
        );

        let url = format!("https://discord.com/api/v9/channels/{}/messages", dm_id);

        let res = self.client.post(url).body(reqwest::Body::from(body)).send().await.unwrap();
        println!("{}", res.text().await.unwrap());
    }
}
