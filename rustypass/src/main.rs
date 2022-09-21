use clap::{Parser, CommandFactory};
use rand::prelude::{SliceRandom, thread_rng};

#[derive(Debug)]
enum PassErr {
    MinLen,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
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

fn main() -> Result<(), PassErr> {
    let mut rng = thread_rng();
    let lower: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g',
                                'h', 'i', 'j', 'k', 'l', 'm', 'n',
                                'o', 'p', 'q', 'r', 's', 't', 'u',
                                'v', 'w', 'x', 'y', 'z'];

    let upper: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G',
                                'H', 'I', 'J', 'K', 'L', 'M', 'N',
                                'O', 'P', 'Q', 'R', 'S', 'T', 'U',
                                'V', 'W', 'X', 'Y', 'Z'];

    let digits: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let special_chars: Vec<char> = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(',
                                        ')', '_', '-', '+', '='];

    const MAGIC_NUM: i32 = 7;
    const MIN_LENGTH: usize = 5;

    let mut charset = Vec::new();
    let cli = Cli::parse();

    if cli.length < MIN_LENGTH {
        eprintln!("cannot generate password below minimum length: {}", MIN_LENGTH);
        Cli::command().print_help();
        return Err(PassErr::MinLen);
    }

    charset.extend(&lower);
    charset.extend(&upper);
    charset.extend(&digits);

    if cli.special {
        charset.extend(&special_chars);
    }

    for _ in 0..MAGIC_NUM {
        charset.shuffle(&mut rng);
    }

    let mut password: Vec<char>;
    loop {
        password = charset.choose_multiple(&mut rng, cli.length).cloned().collect();

        if !(has(&password, &lower) && has(&password, &upper) && has(&password, 
                                                                     &digits)) {
            continue
        }

        if cli.special && !has(&password, &special_chars) {
            continue
        }

        break
    }

    println!("Generated: {}", String::from_iter(password.iter()));
    Ok(())
}
