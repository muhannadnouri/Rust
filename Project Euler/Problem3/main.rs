fn is_prime(value: i64) -> bool {
    for x in 2..value {
        if value % x == 0 {
            return false;
        }
    }
    return value > 1;
}

fn main() {
    const BIG_NUMBER: i64 = 600851475143;
    let mut prime_factors: Vec<i64> = vec![];
    for x in 1..BIG_NUMBER {
        if BIG_NUMBER % x == 0 && is_prime(x) {
            prime_factors.push(x);
        }
    }
    // println!("{:?}", prime_factors);
    let max: i64 = *prime_factors.iter().max().unwrap();
    println!("Max: {}", max);
}
