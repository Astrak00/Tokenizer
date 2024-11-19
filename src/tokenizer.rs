use base64::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;

const URL_TOKEN: &str = "https://aulaglobal.uc3m.es/admin/tool/mobile/launch.php?service=moodle_mobile_app&passport=82.93261629596182&urlscheme=moodlemobile";

pub async fn get_token(cookie: &str) -> Result<String, Box<dyn Error>> {
    // Set the URL and headers
    let mut headers = HeaderMap::new();
    let cookie = format!("MoodleSessionag={}", cookie);
    headers.insert("Cookie", HeaderValue::from_str(&cookie)?);

    // Create a client and send the request
    let client = reqwest::Client::new();
    let response = client.get(URL_TOKEN).headers(headers).send().await;
    let mut token;
    match response {
        Ok(_) => return Err("Request failed".into()),
        Err(error) => {
            let re = regex::Regex::new(r"token=([^&]+)$").unwrap();
            if let Some(captures) = re.captures(&error.to_string()) {
                token = captures.get(1).unwrap().as_str();
                // Remove from =) to the end
                token = token.split(")").collect::<Vec<&str>>()[0];
                let decoded_token = BASE64_STANDARD
                    .decode(token)
                    .expect("Failed to decode base64 token");
                let decoded_token_str_complete = String::from_utf8(decoded_token)
                    .expect("Failed to convert decoded token to string");
                let decoded_token_str = decoded_token_str_complete
                    .split(":::")
                    .collect::<Vec<&str>>()[1];
                return Ok(decoded_token_str.to_string());
            }
        }
    }

    Err("Failed to get token".into())
}
