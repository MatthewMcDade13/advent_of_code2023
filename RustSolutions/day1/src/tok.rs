pub const ONE: &'static str = "one";
pub const TWO: &'static str = "two";
pub const THREE: &'static str = "three";
pub const FOUR: &'static str = "four";
pub const FIVE: &'static str = "five";
pub const SIX: &'static str = "six";
pub const SEVEN: &'static str = "seven";
pub const EIGHT: &'static str = "eight";
pub const NINE: &'static str = "nine";

pub fn tokenize(line: &str) -> Vec<&str> {
    let mut toks = Vec::new();

    let raw = line.as_bytes();

    for i in 0..raw.len() {
        let c = raw[i] as char;

        match c {
            'o' => {
                if let Some(s) = substr(raw, i, ONE.len()) {
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
                        if let Some(s) = substr(raw, i, TWO.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    'h' => {
                        if let Some(s) = substr(raw, i, THREE.len()) {
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
                        if let Some(s) = substr(raw, i, FOUR.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    'i' => {
                        if let Some(s) = substr(raw, i, FIVE.len()) {
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
                        if let Some(s) = substr(raw, i, SIX.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    'e' => {
                        if let Some(s) = substr(raw, i, SEVEN.len()) {
                            toks.push(s);
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                }
            }
            'e' => {
                if let Some(s) = substr(raw, i, EIGHT.len()) {
                    toks.push(s);
                } else {
                    continue;
                }
            }
            'n' => {
                if let Some(s) = substr(raw, i, NINE.len()) {
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
