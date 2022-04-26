use regex::Regex;
use reqwest;
use std::collections::HashMap;
use std::str::FromStr;
use std::time;
use tokio::task;

pub fn format_mess_for_ai(messages_raw: &str, username: &str) -> String {
    let re_message_body = Regex::from_str("\\{\"id\": \"\\d+?\", \"type\": \\d*?, \"content\": .*?\"edited_timestamp\": .*?\\}").unwrap();
    let message_matches = re_message_body.find_iter(messages_raw);
    
    let mess_start = ", \"content\": \"";
    let mess_end = "\", \"channel_id\": \"";
    let author_start = ", \"username\": \"";
    let author_end = "\", \"avatar\": \"";
    let mut message_author_vec = vec![];
    for message_raw in message_matches {
        let message_raw = message_raw.as_str();
        let message = message_raw.split_once(mess_start).unwrap().1.split_once(mess_end).unwrap().0;
        let author = message_raw.split_once(author_start).unwrap().1.split_once(author_end).unwrap().0;
        if author == username {
            message_author_vec.push(format!("You: {}", message));
        } else {
            message_author_vec.push(format!("Friend: {}", message));
        }
    }

    message_author_vec.push("Chat:".to_string());
    message_author_vec.reverse();
    message_author_vec.push("You:".to_string());
    let messages_author = message_author_vec.join("\\n");
    
    messages_author
}

