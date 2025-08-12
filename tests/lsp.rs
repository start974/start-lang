use assert_cmd::Command;
use assert_json_diff::assert_json_include;
use serde_json::{from_str, json, Value};

fn encode_request(request: &Value) -> String {
    let request_str = request.to_string();
    let length = request_str.len();
    format!("Content-Length: {length}\r\n\r\n{request_str}")
}

// Nouvelle version de decode_responses qui gère plusieurs messages LSP.
fn decode_responses(responses: &str) -> Vec<Value> {
    let mut messages = Vec::new();
    let mut data = responses;

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
        messages.push(from_str(body).unwrap());
        data = &data[body_end..];
    }
    messages
}

fn exec_lsp_command(requests: Vec<Value>) -> Vec<Value> {
    let mut cmd = Command::cargo_bin("startlang").unwrap();
    let cmd = cmd.arg("lsp");
    for request in requests {
        cmd.write_stdin(encode_request(&request)).unwrap();
    }

    let responses = cmd.output().unwrap();
    decode_responses(&String::from_utf8_lossy(&responses.stdout))
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
