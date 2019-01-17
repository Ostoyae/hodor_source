#[macro_use]
extern crate clap;

use ansi_term;
use pbr::ProgressBar;
use hodor::HodorT;
use colored::*;
use std::{result::*, time::Duration};

fn main() -> Result<(), reqwest::Error> {
    if cfg!(windows) && !ansi_term::enable_ansi_support().is_ok() {
        colored::control::set_override(false);
    }

    let yml = load_yaml!("../../cli.yml");
    let m = clap::App::from_yaml(yml).get_matches();

    let mut pb = ProgressBar::new(100);
    pb.format("╢▌▌░╟");

    let mut hodor = HodorT::new();

    if let Some(id) = m.value_of("id") {
        println!(
            "id = {:?}",
            id.clone().parse::<u64>().unwrap_or_else(|_| {
                println!(
                    "{}{}{}",
                    "ID".red().bold(),
                    " must be a ",
                    "number >= 0".red()
                );
                std::process::exit(1);
            })
        );
        hodor.insert_form("id", id);
    }
    if m.is_present("cookies") {
        println!("{}", "cookies enabled".bold().blink().green());
        hodor.cookies = true;
    };
    if let Some(a) = m.value_of("amount") {
        println!(
            "amount = {:?}",
            a
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
        hodor.set_goal(a.parse::<u64>().unwrap());

    }

    if !m.is_present("force") {
        println!("Crtl-c if not desired input");
        for _i in 0..110 {
            pb.inc();
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    pb.finish_print("App starting now with Params");



//    hodor.set_url("http://158.69.76.135/level0.php");
    hodor.get_html()?;
    hodor.parse_html();
    hodor.insert_form("holdthedoor", "Submit+Query");
    hodor.fake_post_req()?;

    Ok(())
}
