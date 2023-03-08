use std::{error::Error, fmt::format};
use ansi_term::Color;

use dialoguer::Confirm;
use http::Request;

use hyper::{Body, Client};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = "Yeah. Should be a token here";
    let args = std::env::args().collect::<Vec<_>>();
    let msg_hint = args.get(1);
    use hyper_tls::HttpsConnector;
    let mut prompt = format!(
        "
Generate a message of git commit with obeying following descriptions.

Obey following listed rules.

- The message is in ten words at least
- The message should be limited to one line
- The message is written in English
- The message should follow specification of conventional commits
- Your reply should only contains the message
"
    );

    if let Some(msg_hint) = msg_hint {
        prompt.push_str(&format!(
            "
Consider the hints: {msg_hint}

Consider changed files listed below

- Cargo.toml
- main.rs
"
        ))
    }

    println!("prompt: {prompt}");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .uri("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .method("POST")
        .body(Body::from(
            json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                  {
                    "role": "user",
                    "content": prompt,
                  }
                ],
            })
            .to_string(),
        ))
        .unwrap();

    // Make a GET /ip to 'http://httpbin.org'
    let res = client.request(request).await?;

    // And then, if the request gets a response...

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    let buf = String::from_utf8(buf.to_vec()).unwrap();
    println!("buf: {buf:?}");
    let v: Value = serde_json::from_str(&buf).unwrap();

    let commit_msg = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();
    let command = format!(r#"git commit -m "{commit_msg}""#);
    if Confirm::new().with_prompt(format!("Press enter to run `{}`", Color::Green.paint(command))).default(true).interact()? {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    println!("commit_msg: {commit_msg}");

    Ok(())
}
