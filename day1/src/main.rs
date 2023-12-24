use std::{fs, ops::Index};

mod tok;

extern crate common;

fn main() -> common::Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let inputs: Vec<_> = file.split('\n').collect();
    let inputs = inputs.as_slice();

    let p1_total = part_1(inputs);
    println!("Part 1 Total: {p1_total}");

    let p2_total = part_2(inputs)?;
    println!("Part 2 Total: {p2_total}");

    Ok(())
}

fn part_1(inputs: &[&str]) -> u32 {
    const fn is_digit(c: &char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    let total = inputs.iter().fold(0, |acc, line| {
        if line.len() < 1 {
            return acc;
        };
        let left = line.chars().find(is_digit).unwrap().to_digit(10).unwrap();
        let right = line.chars().rfind(is_digit).unwrap().to_digit(10).unwrap();

        let n = (left * 10) + right;
        acc + n
    });

    total
}

fn part_2(inputs: &[&str]) -> common::Result<u32> {
    let total = inputs.iter().fold(0, |acc, line| {
        if line.len() < 1 {
            return acc;
        }

        let nums: Vec<_> = tokenize(line)
            .iter()
            .filter_map(|tok| parse_number(tok).ok())
            .collect();

        let first = nums.first().unwrap();
        let last = nums.last().unwrap();

        let n = (first * 10) + last;

        acc + n
    });

    Ok(total)
}

fn parse_number(nstr: &str) -> common::Result<u32> {
    let n = match nstr {
        tok::ONE => 1,
        tok::TWO => 2,
        tok::THREE => 3,
        tok::FOUR => 4,
        tok::FIVE => 5,
        tok::SIX => 6,
        tok::SEVEN => 7,
        tok::EIGHT => 8,
        tok::NINE => 9,
        _ => nstr.parse::<u32>()?,
    };
    Ok(n)
}

fn tokenize(line: &str) -> Vec<&str> {
    let mut toks = Vec::new();

    let raw = line.as_bytes();

    for i in 0..raw.len() {
        let c = raw[i] as char;

        match c {
            'o' => {
                if let Some(s) = substr(raw, i, tok::ONE.len()) {
                    toks.push(s);
                } else {
                    continue;
                }
            }
            't' => {
                let peek = if i + 1 >= raw.len() {
                    continue;
                } else {
                    raw[i + 1] as char
                };
                match peek {
                    'w' => {
                        if let Some(s) = substr(raw, i, tok::TWO.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    'h' => {
                        if let Some(s) = substr(raw, i, tok::THREE.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                };
            }
            'f' => {
                let peek = if i + 1 >= raw.len() {
                    continue;
                } else {
                    raw[i + 1] as char
                };
                match peek {
                    'o' => {
                        if let Some(s) = substr(raw, i, tok::FOUR.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    'i' => {
                        if let Some(s) = substr(raw, i, tok::FIVE.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                }
            }
            's' => {
                let peek = if i + 1 >= raw.len() {
                    continue;
                } else {
                    raw[i + 1] as char
                };
                match peek {
                    'i' => {
                        if let Some(s) = substr(raw, i, tok::SIX.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    'e' => {
                        if let Some(s) = substr(raw, i, tok::SEVEN.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                }
            }
            'e' => {
                if let Some(s) = substr(raw, i, tok::EIGHT.len()) {
                    toks.push(s);
                } else {
                    continue;
                }
            }
            'n' => {
                if let Some(s) = substr(raw, i, tok::NINE.len()) {
                    toks.push(s);
                } else {
                    continue;
                }
            }
            '0'..='9' => {
                if let Some(s) = substr(raw, i, 1) {
                    toks.push(s);
                } else {
                    continue;
                }
            }
            _ => continue,
        }
    }

    toks
}

fn substr(src: &[u8], from: usize, len: usize) -> Option<&str> {
    let to = from + len;
    if to > src.len() {
        None
    } else {
        let s = std::str::from_utf8(&src[from..to]).unwrap();
        Some(s)
    }
}
