use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fabric_atelier::{config::Settings, mcp::McpServer};
use tokio::runtime::Runtime;
use std::sync::Arc;

fn bench_concurrent_pattern_lookups(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let settings = Settings::default();
    let server = rt.block_on(async { McpServer::new(settings).await.unwrap() });
    let server = Arc::new(server);
    
    let mut group = c.benchmark_group("concurrent_lookups");
    
    for concurrency in &[1, 10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(concurrency),
            concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut handles = vec![];
                        
                        for _ in 0..concurrency {
                            let server = Arc::clone(&server);
                            let handle = tokio::spawn(async move {
                                // Simulate looking up a pattern
                                let request = serde_json::json!({
                                    "jsonrpc": "2.0",
                                    "id": 1,
                                    "method": "tools/list",
                                    "params": {}
                                });
                                
                                let req: fabric_atelier::mcp::JsonRpcRequest =
                                    serde_json::from_value(request).unwrap();
                                server.handle_request(req).await
                            });
                            handles.push(handle);
                        }
                        
                        for handle in handles {
                            black_box(handle.await.unwrap());
                        }
                    })
                })
            },
        );
    }
    
    group.finish();
}

fn bench_server_initialization(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("server_init", |b| {
        b.iter(|| {
            rt.block_on(async {
                let settings = Settings::default();
                let server = McpServer::new(settings).await.unwrap();
                black_box(server)
            })
        })
    });
}

fn bench_request_handling(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let settings = Settings::default();
    let server = rt.block_on(async { McpServer::new(settings).await.unwrap() });
    
    let mut group = c.benchmark_group("request_handling");
    
    // Initialize request
    group.bench_function("initialize", |b| {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        });
        let req: fabric_atelier::mcp::JsonRpcRequest =
            serde_json::from_value(request).unwrap();
        
        b.iter(|| {
            rt.block_on(async {
                let response = server.handle_request(req.clone()).await;
                black_box(response)
            })
        })
    });
    
    // Tools list request
    group.bench_function("tools_list", |b| {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        });
        let req: fabric_atelier::mcp::JsonRpcRequest =
            serde_json::from_value(request).unwrap();
        
        b.iter(|| {
            rt.block_on(async {
                let response = server.handle_request(req.clone()).await;
                black_box(response)
            })
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_concurrent_pattern_lookups,
    bench_server_initialization,
    bench_request_handling
);
criterion_main!(benches);
