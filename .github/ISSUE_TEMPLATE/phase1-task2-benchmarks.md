# ⚡ Implement Performance Benchmarking with Criterion.rs

## Overview
Add comprehensive performance benchmarking to track key generation speed and regex compilation performance, with regression detection to ensure our recent performance fixes remain effective.

## Tasks

### 1. Criterion.rs Setup
- [ ] Add criterion to dev-dependencies
- [ ] Create `benches/` directory structure
- [ ] Configure benchmark harness in Cargo.toml
- [ ] Add benchmark compilation to CI

### 2. Core Benchmarks
- [ ] **Key Generation Rate**: Keys/second across thread counts
- [ ] **Regex Compilation**: Pattern compilation time
- [ ] **Pattern Matching**: Match performance on various patterns
- [ ] **Memory Usage**: Allocation patterns during generation

### 3. Benchmark Suite
- [ ] `bench_key_generation.rs` - Core key generation speed
- [ ] `bench_regex_performance.rs` - Pattern matching efficiency  
- [ ] `bench_thread_scaling.rs` - Multi-threading performance
- [ ] `bench_memory_usage.rs` - Memory allocation patterns

### 4. CI Integration
- [ ] Add benchmark job to CI workflow
- [ ] Store benchmark results as artifacts
- [ ] Configure performance regression detection
- [ ] Add performance comparison for PRs

## Acceptance Criteria
- [ ] Benchmarks run successfully in CI
- [ ] Performance regression detection active
- [ ] Benchmark results stored and comparable
- [ ] Documentation includes performance guidelines

## Implementation Details

### Cargo.toml Changes
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "key_generation"
harness = false
```

### Example Benchmark Structure
```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_key_generation(c: &mut Criterion) {
    c.bench_function("generate_key_pair", |b| {
        b.iter(|| {
            // Benchmark key generation
        })
    });
}

criterion_group!(benches, bench_key_generation);
criterion_main!(benches);
```

### CI Benchmark Job
```yaml
- name: Run benchmarks
  run: cargo bench --bench key_generation -- --output-format json | tee benchmark_results.json
  
- name: Upload benchmark results
  uses: actions/upload-artifact@v4
  with:
    name: benchmark-results
    path: benchmark_results.json
```

## Key Metrics to Track
1. **Keys/second** - Primary performance metric
2. **Regex compilation time** - Our recent optimization impact
3. **Memory usage** - Resource efficiency  
4. **Thread scaling** - Multi-core performance
5. **Pattern complexity** - Performance vs regex complexity

## Testing
- [ ] Verify benchmarks compile and run
- [ ] Test regression detection with intentional slowdown
- [ ] Confirm CI integration works
- [ ] Validate benchmark result storage

## Timeline
**Estimate:** 2-3 days  
**Priority:** High  
**Phase:** 1

## Labels
`enhancement`, `performance`, `phase-1`, `priority-high`, `ci/cd`

## Dependencies
None - can start immediately

---
*Part of Phase 1: Enhanced CI Foundation - Critical for validating our recent regex performance improvements*