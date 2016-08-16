fn fib(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 2,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let mut total = 0;
    let mut i = 1;

    loop {
        let res = fib(i);
        if res > 4_000_000 {
            break;
        }
        if res % 2 == 0 {
            total += res;
        }
        i += 1;
    }
    println!("sum: {}", total);
}
