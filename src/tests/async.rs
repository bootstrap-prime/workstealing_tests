use async_recursion::async_recursion;

#[async_recursion]
async fn fibonacci(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }

    fibonacci(n - 1).await + fibonacci(n - 2).await
}

#[tokio::main]
async fn main() {
    let num = fibonacci(50).await;

    println!("{}", num);

    // assert_eq!(num, 832040);
}
