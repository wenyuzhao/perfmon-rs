# perfmon-rs

A simple perfmon binding.

## Get started

_examples/sum.rs:_
```rust
fn main() {
    perfmon::initialize();
    perfmon::begin();
    let sum: usize = (0..1000000).collect::<Vec<_>>().iter().sum();
    assert_eq!(sum, 499999500000);
    let results = perfmon::end();
    for (key, value) in results {
        println!("{} = {}", key, value);
    }
}
```

Run:
```console
$ export PERF_EVENTS=PERF_COUNT_HW_CACHE_DTLB:MISS,PERF_COUNT_HW_CACHE_ITLB:MISS
$ cargo run --example sum
   Compiling perfmon v0.1.0 (/home/wenyuz/perfmon-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/examples/sum`
PERF_COUNT_HW_CACHE_ITLB:MISS = 19
PERF_COUNT_HW_CACHE_DTLB:MISS = 7392
```