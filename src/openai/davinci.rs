use regex::Regex;
use reqwest;
use std::collections::HashMap;
use std::time;
use tokio::task;

pub struct Davinci {
    token: String,
    client: reqwest::Client,
}

impl Davinci {
    pub fn from(token: String) -> Davinci {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", &token).as_str()).unwrap(),
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
        Davinci { token, client }
    }

    pub async fn complete_response(&self, prompt: &str) -> reqwest::Response {
        let url = "https://api.openai.com/v1/engines/text-davinci-002/completions";
        let body = format!(
            "{{ \
            \"prompt\": \"{}\", \
            \"max_tokens\": {}, \
            \"best_of\": 1,
            \"frequency_penalty\": 0,
            \"presence_penalty\": 0.6,
            \"stop\": [\"Friend\", \"You\"],
            \"temperature\": 0.9,
            \"top_p\": 1
        }}",
            prompt, prompt.len()
        );
        // println!("{}", &body);
        self.client
            .post(url)
            .body(reqwest::Body::from(body))
            .send()
            .await
            .unwrap()
    }

    pub async fn complete_str(&self, prompt: &str) -> Option<String> {
        let prompt = prompt.replace("\n", "\\n");
        if prompt.len() > 8192 {
            return None
        }

        let response = self.complete_response(&prompt).await;
        let content = response.text().await.unwrap();
        println!("{}", &content);
        let now = time::Instant::now();
        let re_start = Regex::new("\"choices\": \\[\\{\"text\": \"(\\\\n)*").unwrap();
        let completion_start = re_start
            .captures(&content)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();
        let completion_end = "\", \"index\": 0, \"";
        let completion = content
            .split_once(completion_start)
            .unwrap()
            .1
            .split_once(completion_end)
            .unwrap()
            .0
            .replace("\\n", "")
            .replace("\\t", "");
        let time = time::Instant::now() - now;
        println!("Time to parse completion is ms: {}", time.as_millis());
        Some(completion.to_string())
    }
}
