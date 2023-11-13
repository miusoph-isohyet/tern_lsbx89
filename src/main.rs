use std::env;
use std::io::{self, Read};
use std::path::Path;

extern crate image;

fn bf(code: &str) {
    let mut s1 = Vec::new();
    let mut matches = std::collections::HashMap::new();
    let mut tape = vec![0u8; 1_000_000];
    let mut cp = 0;
    let mut p = 0;

    for (i, j) in code.chars().enumerate() {
        match j {
            '[' => s1.push(i),
            ']' => {
                let m = s1.pop().unwrap();
                matches.insert(m, i);
                matches.insert(i, m);
            }
            _ => {}
        }
    }

    while cp < code.len() {
        match code.chars().nth(cp).unwrap() {
            '+' => tape[p] = tape[p].wrapping_add(1),
            '-' => tape[p] = tape[p].wrapping_sub(1),
            ',' => {
                let mut input = [0; 1];
                io::stdin().read_exact(&mut input).unwrap();
                tape[p] = input[0];
            }
            '.' => print!("{}", tape[p] as char),
            '<' => p = p.wrapping_sub(1),
            '>' => p = p.wrapping_add(1),
            '[' => {
                if tape[p] == 0 {
                    cp = matches[&cp];
                }
            }
            ']' => {
                if tape[p] != 0 {
                    cp = matches[&cp];
                }
            }
            _ => {}
        }
        cp = cp.wrapping_add(1);
    }
}

fn run(filename: &str) {
    let img = image::open(filename).unwrap();
    let bytes = img.clone().into_bytes();
    let fuck = "+-,.<>[]";
    let mut b = String::new();

    for &i in bytes.iter() {
        match fuck.chars().nth((i % 9) as usize) {
            Some(ch) => b.push(ch),
            None => break,
        }
    }

    bf(&b);
}

fn encode(filename: &str, brainfuck: &str, output: &str) {
    let img = image::open(filename).unwrap();
    let fuck = "+-,.<>[]";
    let mut bytes = img.clone().into_bytes().to_vec();
    let mut w = String::new();

    for ch in brainfuck.chars() {
        if let Some(_) = fuck.chars().position(|x| x == ch) {
            w.push(ch);
        }
    }

    for (i, j) in w.chars().enumerate() {
        bytes[i] = bytes[i].wrapping_div(9).wrapping_mul(9);
        bytes[i] = bytes[i].wrapping_add(fuck.chars().position(|x| x == j).unwrap() as u8);
        if bytes[i] >= 9 {
            bytes[i] = bytes[i].wrapping_sub(9);
        }
    }

    bytes[w.len()] = bytes[w.len()].wrapping_div(9).wrapping_mul(9).wrapping_add(8);
    if bytes[w.len()] >= 9 {
        bytes[w.len()] = bytes[w.len()].wrapping_sub(9);
    }

    let img_bytes = bytes.into_boxed_slice();
    image::save_buffer(Path::new(output), &img_bytes, img.width(), img.height(), image::ColorType::L8).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        run(&args[1]);
    } else if args.len() == 4 {
        encode(&args[1], &std::fs::read_to_string(&args[2]).unwrap(), &args[3]);
    } else {
        println!("Must pass 1 or 3 arguments");
    }
}
