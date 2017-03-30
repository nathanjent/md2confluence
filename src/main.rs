extern crate markdown;

use std::env;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};

mod confluence;

fn main() {
    let mut input_file = "";
    let mut args = env::args();

    // Consume the first arg. It contains the call.
    args.next();

    loop {
        if let Some(arg) = args.next() {
            match &*arg {
                "-j" => {
                    if let Some(input_file) = args.next() {
                        let file = File::open(input_file).unwrap();
                        let mut reader = BufReader::new(file);
                        let mut buf = String::new();
                        let mut out = String::new();
                        while reader.read_line(&mut buf).unwrap() > 0 {
                            let md = to_markdown(&*buf);
                            out.push_str(&*md);
                            buf.clear();
                        }
                    }
                }
                "-m" => {
                    if let Some(input_file) = args.next() {
                        let mut file = File::open(input_file).unwrap();
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).unwrap();
                        let out = to_confluence(&*buf);
                        print!("{}", out);
                    }
                }
                _ => {
                    println!("Use -c <file> to convert confluence to markdown.");
                    println!("Use -m <file> to convert markdown to confluence.");
                    ::std::process::exit(0);
                }
            }
        } else {
            break;
        }
    }
}

/// Convert a line confluence of to a line of markdown
fn to_markdown(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars();
    loop {
        if let Some(c) = chars.next() {
            match c {
                'h' => {}
                _ => {}
            }
        } else {
            break;
        }
    }
    out
}

/// Convert a line of markdown to a line of confluence
fn to_confluence(input: &str) -> String {
    let md_blocks = markdown::tokenize(input);

    confluence::to_confluence(&md_blocks[..])
}
