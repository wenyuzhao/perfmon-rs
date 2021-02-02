use std::env;

#[inline(never)]
fn random_vec(size: usize) -> Vec<i64> {
    let mut vec = vec![];
    for _ in 0..size {
        vec.push(rand::random());
    }
    vec
}

#[inline(never)]
fn sort_vec<T: Ord>(mut vec: Vec<T>) -> Vec<T> {
    vec.sort();
    vec
}

#[inline(never)]
fn is_sorted<T: Ord>(vec: &Vec<T>) -> bool{
    if vec.is_empty() { return true }
    let mut prev = &vec[0];
    for i in 1..vec.len() {
        if vec[i] < *prev { return false }
        prev = &vec[i]
    }
    true
}

#[test]
fn tlb_miss() {
    env::set_var("PERF_EVENTS", "PERF_COUNT_HW_CACHE_DTLB:MISS,PERF_COUNT_HW_CACHE_ITLB:MISS");
    perfmon::initialize();
    perfmon::begin();
    assert!(is_sorted(&sort_vec(random_vec(1 << 19))));
    let results = perfmon::end();
    println!("{:?}", results);
    assert!(results["PERF_COUNT_HW_CACHE_DTLB:MISS"] > 0);
    assert!(results["PERF_COUNT_HW_CACHE_ITLB:MISS"] > 0);
}