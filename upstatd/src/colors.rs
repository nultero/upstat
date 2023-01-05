use std::collections::HashMap;
use std::fs;
use std::process::exit;

use regex::Regex;

static CONF: &'static str = "/.config/polybar/config.ini";
static DEF: &'static str = "8F8F8F"; // default gray
static DELIM: &'static str = "; *** ;";

pub struct Colors {
    pub map: HashMap<String, String>,
}

impl Colors {
    pub fn polyfmt_print(self: &Self, tag: &str, content: &str) {
        let err = self.map.get(tag);
        let hex: &str;
        match err {
            Some(val) => {
                hex = val;
            }
            None => {
                hex = &DEF;
            }
        }
        print!("%{{F#{}}}{}%{{F-}}", hex, content);
    }

    /// Just prints '|' w/ fmt tags.
    pub fn divider(self: &Self) {
        let err = self.map.get("div");
        let hex: &str;
        match err {
            Some(val) => {
                hex = val;
            }
            None => {
                hex = &DEF;
            }
        }
        print!(" %{{F#{}}}|%{{F-}} ", hex);
    }
}

fn has_delimiter(config: &str) -> Option<usize> {
    for (i, line) in config.lines().enumerate() {
        if line.contains(&DELIM) {
            return Some(i);
        }
    }
    return None;
}

pub fn get_colors(home: &str) -> Colors {
    let mut cs = Colors {
        map: HashMap::with_capacity(50), //I don't think I'll have more?
    };

    let conf_result = fs::read_to_string(format!("{}{}", home, CONF));
    let config: String;
    match conf_result {
        Ok(c) => {
            config = c;
        }
        Err(err) => {
            println!("{}", err);
            return cs;
        }
    }

    let delimiter_idx = has_delimiter(&config);
    let idx: usize; // no delim found
    match delimiter_idx {
        Some(i) => idx = i,
        None => {
            println!("mangled config; colors will not be used");
            return cs;
        }
    }

    let hex_regex: Regex;
    let re_result = Regex::new("#[a-fA-F0-9]{6}$");
    match re_result {
        Ok(r) => hex_regex = r,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }

    for (i, line) in config.lines().enumerate() {
        if line.len() < 2 {
            continue;
        }

        // hit delimiter; quit
        if i >= idx {
            break;
        }

        let mut comment_idx: i32 = -1;
        for (j, chr) in line.chars().enumerate() {
            if chr == ';' {
                comment_idx = j as i32;
                break;
            }
        }
        let mut valid_line = line;
        if comment_idx != -1 {
            valid_line = &valid_line[..comment_idx as usize];
        }

        let m = hex_regex.find(valid_line);
        match m {
            Some(mat) => {
                let splt: Vec<&str> = valid_line.split("=").collect();
                if splt.len() < 2 {
                    continue;
                }
                let hex = valid_line[mat.start() + 1..mat.end()].to_owned();
                let color_str = splt[0].clone();
                let color_str = color_str.trim().to_owned();

                let res = cs.map.insert(color_str, hex);
                // explicit no-ops regardless of whether the key insert works or fails
                match res {
                    Some(_) => {}
                    None => {}
                }
            }
            None => {}
        }
    }
    return cs;
}
