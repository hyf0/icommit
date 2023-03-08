use std::{borrow::Cow, process::Command};

use ansi_term::Color;
use hyper_tls::HttpsConnector;


use anyhow::Result;
use dialoguer::Confirm;
use http::Request;

use hyper::{Body, Client};
use serde_json::{json, Value};
use xshell::{Shell, cmd};


fn staged_files(sh: &Shell) -> Result<Vec<String>> {
    let output = cmd!(&sh, "git diff --name-only --cached").output()?;
    assert!(output.status.success());
    let files = String::from_utf8(output.stdout)?;
    Ok(files.lines().into_iter().map(|s| s.to_string()).collect())
}

type StaticStr = Cow<'static, str>;


#[tokio::main]
async fn main() -> Result<()> {
    let sh = Shell::new()?;

    let verbose = true;

    let staged_files = staged_files(&sh)?;

    if staged_files.is_empty() {
        Err(anyhow::format_err!("No staged files founded"))?
    }

    let token = std::env::var("ICOMMIT_TOKEN")?;
    let args = std::env::args().collect::<Vec<_>>();
    let msg_hint = args.get(1);



    let files_prompt: Vec<StaticStr> = vec![
        "Consider changed files listed below".into(),
        staged_files.into_iter().map(|file| format!("- {file}")).collect::<Vec<_>>().join("\n").into(),
    ];

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

    prompt += &files_prompt.join("\n");

    if let Some(msg_hint) = msg_hint {
        prompt.push_str(&format!(
            "
Consider the hints: {msg_hint}
"
        ))
    }

    if verbose {
        println!("prompt:{}", prompt);
    }

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

    let res = client.request(request).await?;


    let buf = hyper::body::to_bytes(res).await?;
    let buf = String::from_utf8(buf.to_vec()).unwrap();
    if verbose {
        println!("buf: {buf:?}");
    }
    let v: Value = serde_json::from_str(&buf).unwrap();

    let commit_msg = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap()
        .trim()
        .to_string();

    let git_commit_command_for_display = format!(r#"git commit -m "{}""#,  Color::Green.paint(&commit_msg));

    if Confirm::new().with_prompt(git_commit_command_for_display).default(true).interact()? {
        Command::new("git").arg("commit").arg(format!("-m {}", commit_msg)).output().unwrap();
    } else {
        println!("nevermind then :(");
    }

    Ok(())
}


