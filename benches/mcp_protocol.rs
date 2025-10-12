use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fabric_atelier::mcp::protocol::{JsonRpcRequest, JsonRpcResponse};
use serde_json::{json, Value};

fn bench_request_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("request_parsing");
    
    let requests = vec![
        (
            "initialize",
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        ),
        (
            "tools_list",
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
        ),
        (
            "tools_call",
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"fabric_summarize","arguments":{"content":"test content"}}}"#,
        ),
    ];
    
    for (name, request_json) in requests {
        group.bench_with_input(BenchmarkId::from_parameter(name), &request_json, |b, &json| {
            b.iter(|| {
                let request: JsonRpcRequest = serde_json::from_str(json).unwrap();
                black_box(request)
            })
        });
    }
    
    group.finish();
}

fn bench_response_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("response_serialization");
    
    // Success response
    group.bench_function("success_small", |b| {
        let response = JsonRpcResponse::success(json!(1), json!({"status": "ok"}));
        b.iter(|| {
            let json = serde_json::to_string(&response).unwrap();
            black_box(json)
        })
    });
    
    // Success response with large content
    group.bench_function("success_large", |b| {
        let large_content = "x".repeat(10_000);
        let response = JsonRpcResponse::success(
            json!(1),
            json!({
                "content": [{
                    "type": "text",
                    "text": large_content
                }]
            }),
        );
        b.iter(|| {
            let json = serde_json::to_string(&response).unwrap();
            black_box(json)
        })
    });
    
    // Error response
    group.bench_function("error", |b| {
        let response = JsonRpcResponse::error(json!(1), -32601, "Method not found");
        b.iter(|| {
            let json = serde_json::to_string(&response).unwrap();
            black_box(json)
        })
    });
    
    group.finish();
}

fn bench_tools_list_generation(c: &mut Criterion) {
    use fabric_atelier::fabric::PatternLoader;
    use tokio::runtime::Runtime;
    
    let rt = Runtime::new().unwrap();
    let loader = PatternLoader::new().unwrap();
    let patterns = rt.block_on(async { loader.load_all().await.unwrap() });
    
    c.bench_function("generate_tools_list", |b| {
        b.iter(|| {
            let tools: Vec<Value> = patterns
                .iter()
                .map(|pattern| {
                    json!({
                        "name": pattern.tool_name(),
                        "description": pattern.description,
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "content": {
                                    "type": "string",
                                    "description": "The content to process with this pattern"
                                }
                            },
                            "required": ["content"]
                        }
                    })
                })
                .collect();
            black_box(tools)
        })
    });
}

criterion_group!(
    benches,
    bench_request_parsing,
    bench_response_serialization,
    bench_tools_list_generation
);
criterion_main!(benches);
