use async_recursion::async_recursion;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

const FIB_NUMBER: u64 = 40;

fn sync_fibonacci(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }

    sync_fibonacci(n - 1) + sync_fibonacci(n - 2)
}

#[async_recursion]
async fn async_fibonacci(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }

    let n1 = tokio::spawn(async_fibonacci(n - 1));
    let n2 = tokio::spawn(async_fibonacci(n - 2));

    n1.await.unwrap() + n2.await.unwrap()
}

fn async_fib(c: &mut Criterion) {
    c.bench_with_input(BenchmarkId::new("async_fibonacci", 1), &1, |b, &_| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| async_fibonacci(black_box(FIB_NUMBER)));
    });
}

fn sync_fib(c: &mut Criterion) {
    c.bench_function("sync_fibonacci", |b| {
        b.iter(|| sync_fibonacci(black_box(FIB_NUMBER)))
    });
}

criterion_group!(benches, async_fib, sync_fib);
criterion_main!(benches);
