use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use fabric_atelier::{config::Settings, mcp::McpServer};
use tokio::runtime::Runtime;
use std::sync::Arc;
use std::time::Duration;

fn bench_sustained_load(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let settings = Settings::default();
    let server = rt.block_on(async { McpServer::new(settings).await.unwrap() });
    let server = Arc::new(server);
    
    let mut group = c.benchmark_group("sustained_load");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    
    // Simulate sustained request load
    for requests_per_batch in &[10, 50, 100, 500] {
        group.throughput(Throughput::Elements(*requests_per_batch as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_requests", requests_per_batch)),
            requests_per_batch,
            |b, &count| {
                b.iter(|| {
                    rt.block_on(async {
                        let mut handles = vec![];
                        
                        for i in 0..count {
                            let server = Arc::clone(&server);
                            let handle = tokio::spawn(async move {
                                let method = if i % 3 == 0 {
                                    "initialize"
                                } else if i % 3 == 1 {
                                    "tools/list"
                                } else {
                                    "tools/call"
                                };
                                
                                let request = if method == "tools/call" {
                                    serde_json::json!({
                                        "jsonrpc": "2.0",
                                        "id": i,
                                        "method": method,
                                        "params": {
                                            "name": "fabric_summarize",
                                            "arguments": {
                                                "content": "test content for benchmarking"
                                            }
                                        }
                                    })
                                } else {
                                    serde_json::json!({
                                        "jsonrpc": "2.0",
                                        "id": i,
                                        "method": method,
                                        "params": {}
                                    })
                                };
                                
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

fn bench_memory_pressure(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("memory_pressure");
    group.sample_size(50);
    
    // Test with varying content sizes
    for size_kb in &[1, 10, 100, 1000] {
        group.throughput(Throughput::Bytes(*size_kb * 1024));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}kb", size_kb)),
            size_kb,
            |b, &size_kb| {
                let content = "x".repeat((size_kb * 1024) as usize);
                
                b.iter(|| {
                    rt.block_on(async {
                        let settings = Settings::default();
                        let server = McpServer::new(settings).await.unwrap();
                        
                        let request = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": 1,
                            "method": "tools/call",
                            "params": {
                                "name": "fabric_summarize",
                                "arguments": {
                                    "content": &content
                                }
                            }
                        });
                        
                        let req: fabric_atelier::mcp::JsonRpcRequest =
                            serde_json::from_value(request).unwrap();
                        let response = server.handle_request(req).await;
                        black_box(response)
                    })
                })
            },
        );
    }
    
    group.finish();
}

fn bench_pattern_reload(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let settings = Settings::default();
    let server = rt.block_on(async { McpServer::new(settings).await.unwrap() });
    
    c.bench_function("reload_patterns", |b| {
        b.iter(|| {
            rt.block_on(async {
                server.reload_patterns().await.unwrap();
                black_box(())
            })
        })
    });
}

criterion_group!(
    benches,
    bench_sustained_load,
    bench_memory_pressure,
    bench_pattern_reload
);
criterion_main!(benches);
