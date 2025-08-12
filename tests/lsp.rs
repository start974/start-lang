use assert_cmd::cargo::cargo_bin;
use assert_json_diff::assert_json_include;
use serde_json::{from_str, json, Value};
use std::io::{BufRead as _, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::thread;

// Encodes a JSON value into an LSP request string with Content-Length header.
fn encode_request(request: &Value) -> String {
    let request_str = request.to_string();
    let length = request_str.len();
    format!("Content-Length: {length}\r\n\r\n{request_str}")
}

// Reads up to `n` LSP responses from a BufReader, assuming all responses are well-formed.
fn read_responses<R: Read>(reader: &mut BufReader<R>, n: usize) -> Vec<Value> {
    let mut all_responses = Vec::new();

    while all_responses.len() < n {
        // Read headers until a blank line is found.
        let mut headers = String::new();
        loop {
            let mut line = String::new();
            // Assuming the stream will not end abruptly.
            reader.read_line(&mut line).unwrap();
            if line.trim().is_empty() {
                break;
            }
            headers.push_str(&line);
        }

        // Extract Content-Length from headers, assuming the header is always present and valid.
        let content_length = headers
            .lines()
            .find(|line| line.starts_with("Content-Length:"))
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        // Read the body, assuming the stream has enough data.
        let mut body_buffer = vec![0; content_length];
        reader.read_exact(&mut body_buffer).unwrap();

        let body = String::from_utf8(body_buffer).unwrap();
        let message = from_str(&body).unwrap();

        all_responses.push(message);
    }
    all_responses
}

/// Executes an LSP command with a manual binary path and prints all communication.
fn exec_lsp_command(requests: Vec<Value>, n_responses: usize) -> Vec<Value> {
    let mut child = Command::new(cargo_bin!("startlang"))
        .arg("lsp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn startlang command");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    // Create a channel to send results from the reader thread to the main thread.
    let (tx, rx) = std::sync::mpsc::channel();

    // Spawn a thread to handle the asynchronous reading from stdout.
    let reader_thread = thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let responses = read_responses(&mut reader, n_responses);
        tx.send(responses).unwrap();
    });

    // Send all requests to the child's stdin on the main thread.
    println!("========== sending raw requests ==========");
    for request in requests {
        let encoded_request = encode_request(&request);
        println!("{encoded_request}");
        stdin.write_all(encoded_request.as_bytes()).unwrap();
    }
    println!("=========================================");
    drop(stdin); // Close stdin to signal the end of input.

    // Wait for the reader thread to send its result.
    let responses = rx.recv().unwrap();

    // Print the decoded responses.
    println!("--------- received decoded responses ----------");
    for response in &responses {
        println!("{}", serde_json::to_string_pretty(response).unwrap());
    }
    println!("-----------------------------------------");

    let _ = child.wait().unwrap();
    reader_thread.join().unwrap();

    responses
}
#[test]
fn initialize() {
    // write request
    let requests = {
        let initialized = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "rootUri": null,
                "capabilities": {}
            }
        });
        vec![initialized]
    };
    let mut responses = exec_lsp_command(requests, 1).into_iter();
    {
        let server_response = responses.next().unwrap();
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "capabilities": {
                    "textDocumentSync": 1,
                },
                "serverInfo": {
                    "name": "startlang",
                    "version": env!("CARGO_PKG_VERSION")
                },
            }
        });
        assert_json_include!(actual: server_response, expected: response);
    }
}

#[test]
fn initialized_notification() {
    let requests = {
        let initialized_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "rootUri": null,
                "capabilities": {}
            }
        });
        let initialized_notification = json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        });
        vec![initialized_request, initialized_notification]
    };

    let mut responses = exec_lsp_command(requests, 1).into_iter();
    {
        let server_response = responses.next().unwrap();
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "capabilities": {
                    "textDocumentSync": 1,
                },
                "serverInfo": {
                    "name": "startlang",
                    "version": env!("CARGO_PKG_VERSION")
                },
            }
        });
        assert_json_include!(actual: server_response, expected: response);
    }
}
/*
mod did_open {
    use super::*;

    #[test]
    fn no_diagnostic() {
        let requests = {
            let initialized = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "rootUri": null,
                    "capabilities": {}
                }
            });

            let initialized_notification = json!({
                "jsonrpc": "2.0",
                "method": "initialized",
                "params": {}
            });

            let did_open = json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": {
                        "uri": "file:///test.st",
                        "languageId": "startlang",
                        "version": 1,
                        "text": "Def a := 42."
                    }
                }
            });
            vec![initialized, initialized_notification, did_open]
        };
        let mut responses = exec_lsp_command(requests).into_iter();
        let _initialized_response = responses.next().unwrap();
        {
            let server_response = responses.next().unwrap();
            let response = json!({
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": "file:///test.st",
                    "diagnostics": []
                }
            });
            assert_json_include!(actual: server_response, expected: response);
        }
    }
}
*/
