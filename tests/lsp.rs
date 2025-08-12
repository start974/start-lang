use assert_cmd::Command;
use assert_json_diff::assert_json_include;
use serde_json::{from_str, json, Value};

fn encode_request(request: &Value) -> String {
    let request_str = request.to_string();
    let length = request_str.len();
    format!("Content-Length: {length}\r\n\r\n{request_str}")
}

// Nouvelle version de decode_responses qui gère plusieurs messages LSP.
fn decode_responses(mut data: &str) -> Vec<Value> {
    let mut messages = Vec::new();

    while !data.is_empty() {
        //find header
        let header_end = data.find("\r\n\r\n").unwrap();
        let header_str = &data[..header_end];

        // extract content length
        let content_length_str = header_str
            .lines()
            .find(|line| line.starts_with("Content-Length:"))
            .and_then(|line| line.split(':').nth(1))
            .and_then(|s| s.trim().parse::<usize>().ok());

        let content_length = content_length_str.unwrap();
        let body_start = header_end + 4; // remove "\r\n\r\n"
        let body_end = body_start + content_length;

        // parse body
        let body = &data[body_start..body_end];
        let message = from_str(body).unwrap();
        messages.push(message);
        data = &data[body_end..];
    }
    messages
}

fn exec_lsp_command(requests: Vec<Value>) -> Vec<Value> {
    let mut cmd = Command::cargo_bin("startlang").unwrap();
    let cmd = cmd.arg("lsp");
    let request_str = requests
        .iter()
        .map(encode_request)
        .collect::<Vec<String>>()
        .join("");

    // send request
    println!("========== sending raw request ==========");
    println!("{request_str}");
    println!("=========================================");
    cmd.write_stdin(request_str).unwrap();

    // receive response
    let output = cmd.output().unwrap();
    let responses = String::from_utf8_lossy(&output.stdout);
    println!("--------- recive raw response ----------");
    println!("{responses}");
    println!("-----------------------------------------");
    decode_responses(&responses)
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
    let mut responses = exec_lsp_command(requests).into_iter();
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

    let responses = exec_lsp_command(requests);
    // just one response for initialize `initialized`.
    assert_eq!(responses.len(), 1);
}
