fn prime(v: u64) -> bool {
    !(2..).take_while(|x| x * x <= v).any(|x| v % x == 0)
}

fn digits_match(mut s: u64, mut b: u64) -> bool {
    let sdigits = std::iter::from_fn(move || {
        if s == 0 {
            None
        } else {
            let digit = s % 10;
            s = s / 10;
            Some(digit)
        }
    });
    let bdigits = std::iter::from_fn(move || {
        if b == 0 {
            None
        } else {
            let digit = b % 10;
            b = b / 10;
            Some(digit)
        }
    });
    sdigits.zip(bdigits).all(|(a, b)| a == b)
}

fn get_s(p1: u64, p2: u64) -> u64 {
    ((1..)
        .step_by(2)
        .take_while(|m| !digits_match(p1, p2 * m))
        .last()
        .unwrap()
        + 2)
        * p2
}

fn main() {
    let extra_prime = (1000000..).filter(|&v| prime(v)).take(1);
    let primes: Vec<u64> = (5..1000000)
        .filter(|&v| prime(v))
        .chain(extra_prime)
        .collect();
    let skipped_primes = primes.iter().skip(1);
    let zipped = primes.iter().zip(skipped_primes);
    let sum = zipped.fold(0, |acc, (&p1, &p2)| {
        let v = get_s(p1, p2);
        acc + v
    });
    println!("{}", sum);
}

#[test]
fn test_example() {
    let p1 = 19;
    let p2 = 23;
    let s = get_s(p1, p2);
    assert_eq!(s, 1219);
}
