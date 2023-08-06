use std::str::Chars;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /*/// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,*/
}

fn main() {
    let args = Args::parse();

    /*for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }*/
    let xx = capitalize(&args.name);
    println!("{}", xx);
}

fn capitalize(s: &str) -> String{
    /*let words = s.split_whitespace();
    let mut results = "".to_string();

    for word in words{
        let upper = word[0..1].to_uppercase();
        results = results.to_string() + &upper + &word[1..].to_string() + &" ".to_string();
    }*/

    let words: Vec<char> = s.chars().collect();
    let mut results = "".to_string();
    for(i, c) in words.iter().enumerate(){
        if i==0 && c.is_alphabetic()==true{
            results += &c.to_uppercase().to_string();
        }else if words[i-1].is_whitespace()==true && c.is_alphabetic(){
            results += &c.to_uppercase().to_string();
        }else{
            results += &c.to_string();
        }
    }

    results
}

