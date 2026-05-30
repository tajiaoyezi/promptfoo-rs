use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use promptfoo_rs::providers::{
    normalize_provider_request, ProviderConfig, ProviderId, ProviderInput, ProviderRegistry,
};
use serde_json::{json, Value};

#[test]
fn test_4_1_1_p0_provider_registry_has_request_response_snapshots() {
    let registry = ProviderRegistry::register_p0_defaults();
    assert_eq!(
        registry.provider_ids(),
        vec!["anthropic", "http", "ollama", "openai-compatible"]
    );

    let input = ProviderInput::new("Hello from promptfoo-rs");
    let configs = [
        ProviderConfig::openai_compatible("openai-compatible").with_model("gpt-4o-mini"),
        ProviderConfig::http("http").with_base_url("https://example.invalid/provider"),
        ProviderConfig::ollama("ollama").with_model("llama3"),
        ProviderConfig::anthropic("anthropic").with_model("claude-3-haiku-20240307"),
    ];

    let snapshots = configs
        .into_iter()
        .map(|config| {
            registry
                .resolve(&ProviderId::new(config.id.clone()))
                .expect("TEST-4.1.1 P0 provider should resolve");
            registry
                .snapshot(&config, input.clone())
                .expect("TEST-4.1.1 provider snapshot should normalize")
        })
        .collect::<Vec<_>>();

    assert_eq!(
        snapshots[0].request.url,
        "https://api.openai.com/v1/chat/completions"
    );
    assert_eq!(
        snapshots[0].response_output_path,
        "choices[0].message.content"
    );
    assert_eq!(snapshots[1].request.url, "https://example.invalid/provider");
    assert_eq!(snapshots[1].response_output_path, "output");
    assert_eq!(
        snapshots[2].request.url,
        "http://localhost:11434/api/generate"
    );
    assert_eq!(snapshots[2].response_output_path, "response");
    assert_eq!(
        snapshots[3].request.url,
        "https://api.anthropic.com/v1/messages"
    );
    assert_eq!(snapshots[3].response_output_path, "content[0].text");
}

#[test]
fn test_4_1_2_provider_scoped_env_header_model_and_options_are_normalized() {
    let config = ProviderConfig::openai_compatible("openai-compatible")
        .with_model("${OPENAI_MODEL}")
        .with_env("OPENAI_MODEL", "gpt-4o-mini")
        .with_env("OPENAI_API_KEY", "sk-test")
        .with_header("X-Trace-Id", "${TRACE_ID}")
        .with_env("TRACE_ID", "trace-123")
        .with_option("temperature", json!(0))
        .with_option("top_p", json!(0.9));

    let request = normalize_provider_request(config, ProviderInput::new("Summarize this"))
        .expect("TEST-4.1.2 provider request should normalize");

    assert_eq!(request.method, "POST");
    assert_eq!(request.headers["authorization"], "Bearer sk-test");
    assert_eq!(request.headers["x-trace-id"], "trace-123");
    assert_eq!(request.body["model"], "gpt-4o-mini");
    assert_eq!(request.body["temperature"], 0);
    assert_eq!(request.body["top_p"], 0.9);
    assert_eq!(request.body["messages"][0]["content"], "Summarize this");
}

#[tokio::test]
async fn test_4_1_3_network_call_uses_mock_server_not_real_model() {
    let mut server = MockServer::spawn(json!({
        "choices": [
            {
                "message": {
                    "content": "mocked provider response"
                }
            }
        ]
    }));
    let config = ProviderConfig::openai_compatible("openai-compatible")
        .with_base_url(server.url("/v1/chat/completions"))
        .with_model("gpt-4o-mini")
        .with_env("OPENAI_API_KEY", "sk-mock");

    let response = ProviderRegistry::register_p0_defaults()
        .call(&config, ProviderInput::new("Ping mock provider"))
        .await
        .expect("TEST-4.1.3 provider call should use mock server");

    assert_eq!(response.provider_id, "openai-compatible");
    assert_eq!(response.output, "mocked provider response");
    let received = server.received_request();
    assert!(
        received.starts_with("POST /v1/chat/completions "),
        "{received}"
    );
    assert!(
        received.contains("authorization: Bearer sk-mock"),
        "{received}"
    );
    assert!(received.contains("Ping mock provider"), "{received}");
}

struct MockServer {
    address: String,
    received: Arc<Mutex<Option<String>>>,
    handle: Option<JoinHandle<()>>,
}

impl MockServer {
    fn spawn(response: Value) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("mock server should bind");
        let address = listener
            .local_addr()
            .expect("mock server should expose address")
            .to_string();
        let received = Arc::new(Mutex::new(None));
        let received_for_thread = received.clone();
        let response_body = response.to_string();

        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("mock server should accept");
            let request = read_http_request(&mut stream);
            *received_for_thread.lock().unwrap() = Some(request);
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream
                .write_all(response.as_bytes())
                .expect("mock response should write");
        });

        Self {
            address,
            received,
            handle: Some(handle),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.address, path)
    }

    fn received_request(&mut self) -> String {
        if let Some(handle) = self.handle.take() {
            handle.join().expect("mock server thread should finish");
        }
        self.received
            .lock()
            .unwrap()
            .clone()
            .expect("mock server should record request")
    }
}

fn read_http_request(stream: &mut std::net::TcpStream) -> String {
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 512];
    loop {
        let read = stream.read(&mut chunk).expect("request should read");
        if read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(header_end) = find_header_end(&buffer) {
            let headers = String::from_utf8_lossy(&buffer[..header_end]).to_ascii_lowercase();
            let content_length = headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length: "))
                .and_then(|value| value.trim().parse::<usize>().ok())
                .unwrap_or(0);
            let body_start = header_end + 4;
            if buffer.len() >= body_start + content_length {
                break;
            }
        }
    }
    String::from_utf8(buffer).expect("request should be utf8")
}

fn find_header_end(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}
