#![allow(deprecated)]
#![allow(dead_code)]

use assert_cmd::cargo::cargo_bin;
use core::panic;
use fs_extra::dir::CopyOptions;
use startlang::lsp::backend::Backend;
use std::fmt::Debug;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use temp_dir::TempDir;
use tokio::io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout};
use tower_lsp::lsp_types::TextDocumentItem;
use tower_lsp::lsp_types::{notification::Notification, *};
use tower_lsp::{jsonrpc, lsp_types, lsp_types::request::Request};

fn encode_message(content_type: Option<&str>, message: &str) -> String {
    let content_type = content_type
        .map(|ty| format!("\r\nContent-Type: {ty}"))
        .unwrap_or_default();
    format!(
        "Content-Length: {}{}\r\n\r\n{}",
        message.len(),
        content_type,
        message
    )
}

pub struct TestContext {
    pub stdin: ChildStdin,
    pub stdout: BufReader<ChildStdout>,
    pub child: Child,
    pub request_id: i64,
    pub version_id: i32,
    pub workspace: TempDir,
}

impl TestContext {
    pub fn new(base: &str) -> Self {
        let mut child = tokio::process::Command::new(cargo_bin!("startlang"))
            .arg("lsp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn startlang command");

        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());

        // create a temporary workspace an init it with our test inputs
        let workspace = TempDir::new().unwrap();
        for item in fs::read_dir(
            Path::new("tests")
                .join("lsp_tests")
                .join("workspace")
                .join(base),
        )
        .unwrap()
        {
            eprintln!("copying {item:?}");
            fs_extra::copy_items(
                &[item.unwrap().path()],
                workspace.path(),
                &CopyOptions::new(),
            )
            .unwrap();
        }

        Self {
            stdin,
            stdout,
            child,
            request_id: 0,
            version_id: 0,
            workspace,
        }
    }

    pub fn doc_uri(&self, path: &str) -> Url {
        Url::from_file_path(self.workspace.path().join(path)).unwrap()
    }

    pub fn document_item(&mut self, path: &str, text: &str) -> TextDocumentItem {
        self.version_id += 1;
        TextDocumentItem {
            uri: self.doc_uri(path),
            language_id: Backend::name().to_string(),
            version: self.version_id,
            text: text.to_owned(),
        }
    }

    pub fn versioned_document(&mut self, path: &str) -> VersionedTextDocumentIdentifier {
        self.version_id += 1;
        VersionedTextDocumentIdentifier {
            uri: self.doc_uri(path),
            version: self.version_id,
        }
    }

    pub async fn send(&mut self, request: &jsonrpc::Request) {
        let content = serde_json::to_string(request).unwrap();
        eprintln!(
            ">>>>> sending >>>>>>\n{}\n>>>>>>>>>>>>>>>>>>>>",
            serde_json::to_string_pretty(request).unwrap()
        );
        std::io::stderr().flush().unwrap();
        self.stdin
            .write_all(encode_message(None, &content).as_bytes())
            .await
            .unwrap();
    }

    pub async fn response<R: std::fmt::Debug + serde::de::DeserializeOwned>(&mut self) -> R {
        loop {
            // first line is the content length header
            let mut clh = String::new();
            self.stdout.read_line(&mut clh).await.unwrap();
            if !clh.starts_with("Content-Length") {
                panic!("missing content length header");
            }
            let length = clh
                .trim_start_matches("Content-Length: ")
                .trim()
                .parse::<usize>()
                .unwrap();
            // next line is just a blank line
            self.stdout.read_line(&mut clh).await.unwrap();
            // then the message, of the size given by the content length header
            let mut content = vec![0; length];
            self.stdout.read_exact(&mut content).await.unwrap();
            let content = String::from_utf8(content).unwrap();
            std::io::stderr().flush().unwrap();
            // skip log messages
            if content.contains("window/logMessage") {
                continue;
            }
            let response = serde_json::from_str::<jsonrpc::Response>(&content).unwrap();
            let (_id, result) = response.into_parts();
            eprintln!(
                ">>>>> response >>>>>>\n{}\n>>>>>>>>>>>>>>>>>>>>>",
                serde_json::to_string_pretty(&result).unwrap()
            );
            return serde_json::from_value(result.unwrap()).unwrap();
        }
    }

    pub async fn request<R: Request>(&mut self, params: R::Params) -> R::Result
    where
        R::Result: Debug,
    {
        let request = jsonrpc::Request::build(R::METHOD)
            .id(self.request_id)
            .params(serde_json::to_value(params).unwrap())
            .finish();
        self.request_id += 1;
        self.send(&request).await;
        self.response().await
    }

    pub async fn shutdown(&mut self) {
        let request = jsonrpc::Request::build(lsp_types::request::Shutdown::METHOD)
            .id(self.request_id)
            .finish();
        self.request_id += 1;
        self.send(&request).await;
        // we don't care about the response, just that it is sent
    }

    pub async fn recv<R: std::fmt::Debug + serde::de::DeserializeOwned>(&mut self) -> R {
        loop {
            // first line is the content length header
            let mut clh = String::new();
            self.stdout.read_line(&mut clh).await.unwrap();
            if !clh.starts_with("Content-Length") {
                panic!("missing content length header");
            }
            let length = clh
                .trim_start_matches("Content-Length: ")
                .trim()
                .parse::<usize>()
                .unwrap();
            // next line is just a blank line
            self.stdout.read_line(&mut clh).await.unwrap();
            // then the message, of the size given by the content length header
            let mut content = vec![0; length];
            self.stdout.read_exact(&mut content).await.unwrap();
            let content = String::from_utf8(content).unwrap();
            std::io::stderr().flush().unwrap();
            // skip log messages
            if content.contains("window/logMessage") {
                continue;
            }
            let response = serde_json::from_str::<jsonrpc::Request>(&content).unwrap();
            let (_method, _id, params) = response.into_parts();
            let params = params.expect("missing params in response");
            eprintln!(
                ">>>>> recieve >>>>>>\n{}\n>>>>>>>>>>>>>>>>>>>>",
                serde_json::to_string_pretty(&params).unwrap()
            );
            return serde_json::from_value(params).unwrap();
        }
    }

    pub async fn notify<N: Notification>(&mut self, params: N::Params) {
        let notification = jsonrpc::Request::build(N::METHOD)
            .params(serde_json::to_value(params).unwrap())
            .finish();
        self.send(&notification).await;
    }

    pub async fn initialize(&mut self) {
        // a real set of initialize param from helix. We just have to change the workspace configuration
        let initialize = r#"{
        "capabilities": {
          "general": {
            "positionEncodings": [
              "utf-8",
              "utf-32",
              "utf-16"
            ]
          },
          "textDocument": {
            "codeAction": {
              "codeActionLiteralSupport": {
                "codeActionKind": {
                  "valueSet": [
                    "",
                    "quickfix",
                    "refactor",
                    "refactor.extract",
                    "refactor.inline",
                    "refactor.rewrite",
                    "source",
                    "source.organizeImports"
                  ]
                }
              },
              "dataSupport": true,
              "disabledSupport": true,
              "isPreferredSupport": true,
              "resolveSupport": {
                "properties": [
                  "edit",
                  "command"
                ]
              }
            },
            "completion": {
              "completionItem": {
                "deprecatedSupport": true,
                "insertReplaceSupport": true,
                "resolveSupport": {
                  "properties": [
                    "documentation",
                    "detail",
                    "additionalTextEdits"
                  ]
                },
                "snippetSupport": true,
                "tagSupport": {
                  "valueSet": [
                    1
                  ]
                }
              },
              "completionItemKind": {}
            },
            "hover": {
              "contentFormat": [
                "markdown"
              ]
            },
            "inlayHint": {
              "dynamicRegistration": false
            },
            "publishDiagnostics": {
              "tagSupport": {
                "valueSet": [
                  1,
                  2
                ]
              },
              "versionSupport": true
            },
            "rename": {
              "dynamicRegistration": false,
              "honorsChangeAnnotations": false,
              "prepareSupport": true
            },
            "signatureHelp": {
              "signatureInformation": {
                "activeParameterSupport": true,
                "documentationFormat": [
                  "markdown"
                ],
                "parameterInformation": {
                  "labelOffsetSupport": true
                }
              }
            }
          },
          "window": {
            "workDoneProgress": true
          },
          "workspace": {
            "applyEdit": true,
            "configuration": true,
            "didChangeConfiguration": {
              "dynamicRegistration": false
            },
            "didChangeWatchedFiles": {
              "dynamicRegistration": true,
              "relativePatternSupport": false
            },
            "executeCommand": {
              "dynamicRegistration": false
            },
            "fileOperations": {
              "didRename": true,
              "willRename": true
            },
            "inlayHint": {
              "refreshSupport": false
            },
            "symbol": {
              "dynamicRegistration": false
            },
            "workspaceEdit": {
              "documentChanges": true,
              "failureHandling": "abort",
              "normalizesLineEndings": false,
              "resourceOperations": [
                "create",
                "rename",
                "delete"
              ]
            },
            "workspaceFolders": true
          }
        },
        "clientInfo": {
          "name": "helix",
          "version": "24.3 (109f53fb)"
        },
        "processId": 28774,
        "rootPath": "/Users/glehmann/src/earthlyls",
        "rootUri": "file:///Users/glehmann/src/earthlyls",
        "workspaceFolders": [
          {
            "name": "sdk",
            "uri": "file:///Users/glehmann/src/earthlyls"
          }
        ]
      }"#;
        let mut initialize: <lsp_types::request::Initialize as Request>::Params =
            serde_json::from_str(initialize).unwrap();
        let workspace_url = Url::from_file_path(self.workspace.path()).unwrap();
        initialize.root_path = Some(self.workspace.path().to_string_lossy().to_string());
        initialize.root_uri = Some(workspace_url.clone());
        initialize.workspace_folders = Some(vec![WorkspaceFolder {
            name: "tmp".to_owned(),
            uri: workspace_url.clone(),
        }]);
        self.request::<lsp_types::request::Initialize>(initialize)
            .await;
        self.notify::<lsp_types::notification::Initialized>(InitializedParams {})
            .await;
    }
}
