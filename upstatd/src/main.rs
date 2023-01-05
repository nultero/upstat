// mod net;

mod clock;
mod colors;
mod sysbat;
mod updt;

use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn dump_explanation_to_exit() {
    println!("expecting: '--user=<name>' (no <>)");
    exit(1);
}

/// Dumb, but simple + no external deps way to get $HOME
///
/// Won't even hit weird issues w/ boot scripts launching
/// as the wrong user / w/ wrong env if you just pass $user in
fn get_home(arg: String) -> String {
    let parts: Vec<&str> = arg.split("=").collect();
    if parts.len() < 2 {
        println!("argument '{}' mangled?", arg);
        dump_explanation_to_exit();
    }
    let user: &str = parts[1];
    let home = format!("/home/{}", user);
    return home;
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("no user in args");
        dump_explanation_to_exit();
    }

    let home_dir = get_home(args[0].clone());
    let colors = colors::get_colors(&home_dir);

    /*
     Everything from here on determines
     ordering on polybar's handle, which
     should be `tail = true`'d in the config
    */
    let second = Duration::from_secs(1);
    loop {
        sysbat::print_bat(&colors);
        updt::update_bell(&colors, &home_dir);
        // clock::print_time(&colors);
        // net::print_rss_things();
        print!("\n"); //newline term for poly
        sleep(second);
    }
}
