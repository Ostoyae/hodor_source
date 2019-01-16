#[macro_use]
extern crate clap;
extern crate ansi_term;

use colored::*;
use pbr::ProgressBar;
use std::time::Duration;

/// learning to use clap using yaml feature
fn main() {
    read_args();
}


fn read_args() {
    /// Run Clap and then checks the argument is present.
    let mut pb = ProgressBar::new(100);
    pb.format("╢▌▌░╟");

    if cfg!(windows) && !ansi_term::enable_ansi_support().is_ok() {
        colored::control::set_override(false);
    }
    let yml = load_yaml!("../cli.yml");
    let m = clap::App::from_yaml(yml).get_matches();

    if let Some(_id) = m.value_of("id") {
        println!(
            "id ? {:?}",
            m.value_of("id")
                .unwrap()
                .parse::<u32>()
                .unwrap_or_else(|_| {
                    println!(
                        "{}{}{}",
                        "ID".red().bold(),
                        " must be a ",
                        "number >= 0".red()
                    );
                    std::process::exit(1);
                })
        );
    } else {
        println!("ID set to default: {}", "538".bold().blue());
    }
    if let Some(_c) = m.value_of("cookie") {
        println!("cookies ? {:?}", m.is_present("cookies"))
    };

    if let Some(_a) = m.value_of("amount") {
        println!(
            "amount ? {:?}",
            m.value_of("amount")
                .unwrap()
                .parse::<u64>()
                .unwrap_or_else(|_| {
                    println!(
                        "{}{}{}",
                        "Amount".red().bold(),
                        " must be a ",
                        "number >= 0".red()
                    );
                    std::process::exit(1);
                })
        );
    } else {
        println!("Amount set to default: {}", "1024".bold().blue());
    }

    if !m.is_present("force") {
        println!("Crtl-c if not desired input");
        for _i in 0..110 {
            pb.inc();
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    pb.finish_print("App starting now with Params")
}
