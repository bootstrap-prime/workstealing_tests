fn fibonacci(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let num = fibonacci(50);

    println!("{}", num);

    // assert_eq!(num, 832040);
}
