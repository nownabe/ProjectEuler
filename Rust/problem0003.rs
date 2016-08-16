fn main() {
    let mut num: u64 = 600851475143;
    let mut prime = 2;

    loop {
        if num % prime == 0 {
            num /= prime;
            if num < prime { break; }
        } else {
            prime += 1;
        }
    }
    println!("The largest prime factor is {}", prime);
}
