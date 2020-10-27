fn generate_triangle<'a>(s: &'a str) -> impl DoubleEndedIterator<Item = Vec<usize>> + 'a {
    s.trim().split('\n').map(|l| {
        l.split_ascii_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect()
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please supply an input file as argument 1");
    }
    let input_string = std::fs::read_to_string(&args[1]).expect("Failed to load input");
    let mut triangle = generate_triangle(&input_string).rev();
    let acc = triangle.next().expect("the triangle is empty :(");
    let val = triangle
        .fold(acc, |a, mut c| {
            assert!(a.len() == c.len() + 1);
            for (i, v) in c.iter_mut().enumerate() {
                let m = std::cmp::max(a[i], a[i + 1]);
                *v += m;
            }
            c
        })
        .pop()
        .expect("No value returned from the fold");
    println!("{}", val);
}
