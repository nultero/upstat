use chrono::{self, Datelike};
use std::fs::read;

use crate::colors;

const UPDTFILE: &'static str = "/.updt_last_run.txt";

/// The original strftime has the day zero-padded.
fn fmt_day() -> String {
    let now = chrono::Local::now();
    let d = now.day();
    let dt = now.format("%Y %b").to_string();
    let fmtd = format!("{} {}", dt, d);
    return fmtd;
}

fn day_as_bytes() -> Vec<u8> {
    let d = fmt_day();
    return d.as_bytes().to_vec();
}

fn get_updt_file(home: &str) -> (u8, Vec<u8>) {
    let updt = format!("{}{}", home, UPDTFILE);
    let res = read(updt);
    match res {
        Ok(bytes) => {
            return (1, bytes);
        }
        Err(_) => {
            return (0, vec![]);
        }
    }
}

fn already_updated(home: &str) -> bool {
    let d = day_as_bytes();
    let (n, updt) = get_updt_file(home);
    if n == 0 {
        return false;
    }

    // updt file has newline on the end
    if d.len() + 1 != updt.len() {
        return false;
    }

    for (idx, byte) in d.iter().enumerate() {
        if &updt[idx] != byte {
            return false;
        }
    }

    return true;
}

pub fn update_bell(cs: &colors::Colors, home: &str) {
    if !already_updated(home) {
        cs.divider();
        cs.polyfmt_print("alert", "");
    }
}
