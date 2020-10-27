fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please supply an input file as argument 1");
    }
    let input_string = std::fs::read_to_string(&args[1]).expect("Failed to load input");
    let mut names: Vec<&str> = input_string
        .trim()
        .split(",")
        .map(|s| s.trim_matches('"'))
        .collect();
    names.sort();
    let (val, _) = names.iter().fold((0u64, 1u64), |(a, c), v| {
        (
            a + c * v.as_bytes().iter().fold(0u64, |a, c| a + *c as u64 - 64),
            c + 1,
        )
    });
    println!("{}", val);
}
