use colored::Colorize;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) || args.len() == 1 {
        println!(" - USAGE - ");
        println!("The first argument is the file name/path");
        println!("The second argument is the pattern that you want to find in the file");
        println!(" ");
        println!("Example: thegrep example1.txt pattern");
        println!("You can check lines above and bellow using -A and -B");
        println!("Example: thegrep example1.txt pattern -A 10 -B 5A");
    } else {
        grep_document(&args);
    }
}

fn grep_document(args: &[String]) {
    let mut n_line: usize = 0;
    let document = fs::read_to_string(&args[1]).unwrap();
    let mut document_by_lines: Vec<&str> = vec![];

    for line in document.lines() {
        document_by_lines.push(line);
    }

    for line in &document_by_lines {
        n_line += 1;

        if line.contains(&args[2]) {
            if let Some(index) = args.iter().position(|pos| pos == "-B") {
                if let Ok(n) = args[index + 1].parse::<isize>() {
                    for i in (1..n + 1).rev() {
                        if n_line as isize - i < 1 {
                            continue;
                        }
                        println!("{}", document_by_lines[n_line - i as usize - 1])
                    }
                }
            }

            println!("{}", line.red());

            if let Some(index) = args.iter().position(|pos| pos == "-A") {
                if let Ok(n) = args[index + 1].parse::<usize>() {
                    for i in 0..n {
                        if i + n_line < document_by_lines.len() {
                            println!("{}", document_by_lines[i + n_line])
                        }
                    }
                }
            }
        }
    }
}
