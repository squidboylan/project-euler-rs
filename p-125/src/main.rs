fn palindromic(mut v: u64) -> bool {
    let digits = std::iter::from_fn(move || {
        if v == 0 {
            None
        } else {
            let digit = v % 10;
            v = v / 10;
            Some(digit)
        }
    });
    let mut divisor = 1;
    while v / divisor > 9 {
        divisor *= 10;
    }
    let rev_digits = std::iter::from_fn(move || {
        if v == 0 {
            None
        } else {
            let digit = v / divisor;
            v = v % divisor;
            divisor /= 10;
            Some(digit)
        }
    });
    digits.zip(rev_digits).all(|(a, b)| a == b)
}

fn sum_of_consecutive_squares(v: u64) -> bool {
    (1..).take_while(|&x| x * x < v).any(|x| {
        let mut sum = 0;
        for y in x.. {
            sum += y * y;
            if sum == v {
                return true;
            } else if sum > v {
                return false;
            }
        }
        unreachable!();
    })
}

fn main() {
    let palindromic_nums = (2..10_u64.pow(8)).filter(|&v| palindromic(v));

    let nums = palindromic_nums.filter(|&v| sum_of_consecutive_squares(v));

    let sum = nums.fold(0, |a, v| a + v);
    println!("{}", sum);
}
