use clap::Parser;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args{
    /// Use special characters [default: false]
    #[clap(short, long, action)]
    special: bool,

    /// Length of generated password
    #[clap(short, long, value_parser, default_value_t = 10)]
    length: usize,
}

// Return if s2 contains any element from s1
fn has<T: std::cmp::PartialEq>(s1: &Vec<T> , s2: &Vec<T>) -> bool {
    return s1.iter().any(|e| s2.contains(e))
}

fn main() {
    let mut rng = thread_rng();
    let lower: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let upper: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let digits: Vec<char> = "0123456789".chars().collect();
    let special_chars: Vec<char> = "!@#$%^&*()_-+=".chars().collect();

    const MAGIC_NUM: i32 = 7;

    let mut charset = Vec::new();
    let args = Args::parse();

    charset.extend(&lower);
    charset.extend(&upper);
    charset.extend(&digits);

    if args.special {
        charset.extend(&special_chars);
    }

    for _ in 0..MAGIC_NUM {
        charset.shuffle(&mut rng);
    }

    let mut password: Vec<char>;
    loop {
        password = charset.choose_multiple(&mut rng, args.length).
        cloned().collect();

        if !has(&password, &lower) && !has(&password, &upper) && !has(&password, &digits) {
            continue
        }

        if args.special && !has(&password, &special_chars) {
            continue
        }

        break
    }

    println!("Generated: {}", String::from_iter(password.iter()));
}
