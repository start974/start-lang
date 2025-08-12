use assert_cmd::Command;
use assert_json_diff::assert_json_include;
use serde_json::{from_str, json};

fn encode_request(request: &serde_json::Value) -> String {
    let request_str = request.to_string();
    let length = request_str.len();
    format!("Content-Length: {length}\r\n\r\n{request_str}")
}

fn decode_response(response: &str) -> serde_json::Value {
    let mut parts = response.split("\r\n\r\n");
    let _ = parts.next();
    let body = parts.next().unwrap();
    from_str(body).unwrap()
}

#[test]
fn initialize() {
    let mut cmd = Command::cargo_bin("startlang").unwrap();
    let child = cmd.arg("lsp");

    // write request
    {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "rootUri": null,
                "capabilities": {}
            }
        });

        child.write_stdin(encode_request(&request)).unwrap();
    }

    {
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
        let output = child.output().unwrap();

        // read response
        let stdout = String::from_utf8_lossy(&output.stdout);
        let server_response = decode_response(&stdout);
        assert_json_include!(actual: server_response, expected: response);
    }
}
