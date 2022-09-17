use std::{fs::File, io::Read};

static ENG_NOW: &'static str = "/sys/class/power_supply/BAT0/energy_now";
static ENG_FUL: &'static str = "/sys/class/power_supply/BAT0/energy_full";
static LIGHTNING_BOLT: char = '⚡';
static THRESHOLDS: [f32;  5] = [ 98.9, 88.0, 70.0, 50.0, 24.0 ];
static UCHARS:     [char; 6] = [ ' ', '', '', '', '', '' ];

fn get_val(path: &'static str, buf: &mut [u8; 20]) -> f32 {
    let mut f = File::open(path).unwrap();
    let n = f.read(buf).unwrap();
    let s = String::from_utf8(
        buf[..n-1].to_vec() // slice off newline, causes issues with Rust's parser
    ).unwrap();
    let val: f32 = s.parse().unwrap();
    return val;
}

fn main() {
    let mut buf: [u8; 20] = [0; 20];
    let enow = get_val(ENG_NOW, &mut buf);
    let eful = get_val(ENG_FUL, &mut buf);
    let used_pc = (enow / eful) * 100.0;

    if used_pc > THRESHOLDS[0] {
        // print!("{}", LIGHTNING_BOLT); //<- regular print
        // polybar color fmt
        print!("%{{F#44ff00}}{}%{{F-}}", LIGHTNING_BOLT);
        return;
    }
    
    for i in 1..5 {
        if used_pc > THRESHOLDS[i] {
            print!("{:.2} {}", used_pc, UCHARS[i]);
            return;
        }
    }

    print!("{:.2} {}", used_pc, UCHARS[5]);
}