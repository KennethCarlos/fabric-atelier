use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fabric_atelier::fabric::PatternLoader;
use tokio::runtime::Runtime;

fn bench_pattern_loading(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("load_all_patterns", |b| {
        b.iter(|| {
            rt.block_on(async {
                let loader = PatternLoader::new().unwrap();
                let patterns = loader.load_all().await.unwrap();
                black_box(patterns)
            })
        })
    });
}

fn bench_pattern_search(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let loader = PatternLoader::new().unwrap();
    let patterns = rt.block_on(async { loader.load_all().await.unwrap() });
    
    let mut group = c.benchmark_group("pattern_search");
    
    // Benchmark different search queries
    for query in &["write", "security", "analyze", "extract", "summarize"] {
        group.bench_with_input(BenchmarkId::from_parameter(query), query, |b, &query| {
            b.iter(|| {
                let results: Vec<_> = patterns
                    .iter()
                    .filter(|p| p.matches(query))
                    .collect();
                black_box(results)
            })
        });
    }
    
    group.finish();
}

fn bench_pattern_access(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let loader = PatternLoader::new().unwrap();
    let patterns = rt.block_on(async { loader.load_all().await.unwrap() });
    
    c.bench_function("find_pattern_by_name", |b| {
        b.iter(|| {
            let pattern = patterns.iter().find(|p| p.name == "summarize");
            black_box(pattern)
        })
    });
    
    c.bench_function("get_tool_names", |b| {
        b.iter(|| {
            let names: Vec<_> = patterns.iter().map(|p| p.tool_name()).collect();
            black_box(names)
        })
    });
}

criterion_group!(
    benches,
    bench_pattern_loading,
    bench_pattern_search,
    bench_pattern_access
);
criterion_main!(benches);
