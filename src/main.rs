use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) || args.len() == 1 {
        println!(" - USAGE - ");
        println!("The first argument is the file name/path");
        println!("The second argument is the pattern that you want to find in the file");
        println!("");
        println!("Example: thegrep example.txt pattern");
    } else {
        grep_document(&args);
    }
}

fn grep_document(args: &Vec<String>) {
    let mut n_line = 0;
    let document = fs::read_to_string(&args[1]).unwrap();

    for line in document.lines() {
        n_line = n_line + 1;
        if line.contains(&args[2]) {
            println!("{}", line);
            // if args.contains(&"-A".to_string()) {
            //     println!("{}", "hola")
            // }

            if let Some(index) = args.iter().position(|arg| arg == "-A") {
                if let Ok(n) = args[index + 1].parse::<u8>() {
                    println!("{}", n);
                }
            }
        }
    }
}
