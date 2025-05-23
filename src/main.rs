use std::env;
use std::fs;

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    let (modifiers, filenames): (Vec<_>, Vec<_>) = arguments
        .into_iter()
        .partition(|x| x.chars().nth(0) == Some('-'));

    println!("{:?}", modifiers);
    println!("{:?}", filenames);
    println!("\n\n\n");

    let mut print_ends: bool = false;
    let mut print_line_nums: bool = false;
    let merged_modifiers = modifiers.iter().map(|s| s.chars()).flatten();

    for modi in merged_modifiers {
        match modi {
            'e' => print_ends = true,
            'n' => print_line_nums = true,
            '-' => continue,
            _ => {
                println!("Invalid Arguments");
                return;
            }
        }
    }

    let mut new_line: bool = true;
    let mut i = 0;

    for filename in filenames {
        let contents = fs::read_to_string(filename.clone()).expect("wrong path");
        for character in contents.chars() {
            if new_line && print_line_nums {
                i += 1;
                print!("{}  ", i);
                new_line = false;
            }
            if character == '\n' {
                if print_ends {
                    print!("$");
                }
                new_line = true;
            }
            print!("{}", character);
        }
    }
}
