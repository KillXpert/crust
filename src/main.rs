use std::env;
use std::fs;

fn linecount(filename: &str) -> u32 {
    let mut lc = 0;
    for c in filename.chars() {
        if c == '\n' {
            lc += 1;
        }
    }
    return lc;
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    let (modifiers, filename): (Vec<_>, Vec<_>) = arguments
        .into_iter()
        .partition(|x| x.chars().nth(0) == Some('-'));
    println!("{:?}", modifiers);
    println!("{:?}", filename);
    println!("\n\n\n");

    let contents = fs::read_to_string(filename[0].clone()).expect("wrong path");
    let lc = linecount(&contents);

    if modifiers.len() == 0 {
        print!("{}", contents);
    }

    for modi in modifiers {
        match modi.as_ref() {
            "-n" => {
                let mut i: u32 = 1;
                if contents.len() > 0 {
                    print!("1  ")
                }
                for character in contents.chars() {
                    print!("{}", character);
                    if character == '\n' && i < lc {
                        i += 1;
                        print!("{}  ", i);
                    }
                }
            }
            "-e" => {
                for character in contents.chars() {
                    if character == '\n' {
                        print!("$");
                    }
                    print!("{}", character);
                }
            }
            "-ne" => {
                let mut condition: bool = true;
                let mut i = 0;
                for character in contents.chars() {
                    if condition {
                        i += 1;
                        print!("{}  ", i);
                        condition = false;
                    }
                    if character == '\n' {
                        print!("$");
                        condition = true;
                    }
                    print!("{}", character);
                }
            }
            "-en" => {
                let mut condition: bool = true;
                let mut i = 0;
                for character in contents.chars() {
                    if condition {
                        i += 1;
                        print!("{}  ", i);
                        condition = false;
                    }
                    if character == '\n' {
                        print!("$");
                        condition = true;
                    }
                    print!("{}", character);
                }
            }
            &_ => println!("invalid arguments"),
        }
    }
}
