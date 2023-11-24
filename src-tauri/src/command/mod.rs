use tauri::Window;
use crate::status;
use std::{io};
use std::io::{BufRead};
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::sync::Arc;

pub const CONVERTER_EVENT: &str = "converter-event";
pub const CONVERTER_STOP_EVENT: &str = "converter-stop";

#[derive(Clone, Serialize, Deserialize)]
pub struct Payload {
    pub progress: f64,
    pub status: String,
    pub logs: String,
    pub error: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Value {
    pub text: String,
    pub error: String,
    pub status: String,
}

pub fn command<F: FnMut(Value), R: tauri::Runtime>(window: &Window<R>, stop_event: &str, execute_path: &str, args: Vec<&str>, mut on_value: F) {
    let cmd = Command::new(execute_path)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match cmd {
        Ok(cmd) => {
            let cmd = Arc::new(Mutex::new(cmd));

            // Terminate the process
            let cmd_clone = Arc::clone(&cmd);
            let handler = window.once(stop_event, move |_event| {
                let mut cmd = cmd_clone.lock().unwrap();
                if let Err(e) = cmd.kill() {
                    #[cfg(debug_assertions)]
                    {
                        println!("[ERROR] Failed to terminate process: {}", e);
                    }
                } else {
                    #[cfg(debug_assertions)]
                    {
                        println!("[INFO] Process terminated successfully.");
                    }
                }
            });

            let stderr = cmd.as_ref().lock().unwrap().stderr.take(); // Take ownership of stderr
            match stderr {
                Some(stderr) => {
                    let reader = io::BufReader::new(stderr);

                    for line in reader.lines() {
                        let line = line.unwrap();
                        on_value(Value {
                            text: line,
                            error: String::new(),
                            status: String::from(status::IN_PROGRESS),
                        })
                    }

                    let command_status = cmd.as_ref().lock().unwrap().wait();
                    match command_status {
                        Ok(status) => {
                            if status.success() {
                                on_value(Value {
                                    text: String::new(),
                                    error: String::new(),
                                    status: String::from(status::DONE),
                                })
                            } else {
                                on_value(Value {
                                    text: String::new(),
                                    error: String::from("Exit error"),
                                    status: String::from(status::ERROR),
                                })
                            }
                        }
                        Err(e) => {
                            on_value(Value {
                                text: String::new(),
                                error: e.to_string(),
                                status: String::from(status::ERROR),
                            })
                        }
                    }
                }
                None => {
                    on_value(Value {
                        text: String::new(),
                        error: String::from("No result"),
                        status: String::from(status::ERROR),
                    })
                }
            }

            window.unlisten(handler);
        }
        Err(e) => {
            on_value(Value {
                text: String::new(),
                error: e.to_string(),
                status: String::from(status::ERROR),
            })
        }
    }
}