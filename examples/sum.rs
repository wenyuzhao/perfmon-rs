
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