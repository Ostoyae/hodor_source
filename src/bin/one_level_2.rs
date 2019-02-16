#[macro_use]
extern crate clap;

use std::{result::*, time::Duration};

#[cfg(target_os = "windows")]
use ansi_term;
use colored::*;
use pbr::ProgressBar;
use reqwest::header::*;

use hodor::HodorT;

fn main() -> Result<(), reqwest::Error> {
    enable_ansi();

    let mut hodor = HodorT::new();
    hodor = run_clap(hodor);

    hodor.set_url("http://158.69.76.135/level2.php");

    let client = reqwest::Client::new();

    let ck = hodor.get_cookie(&client);
    let mut header = HeaderMap::new();

    header.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"),
    );

    //    Deal with Cookie
    let v = ck.get("HoldTheDoor").unwrap();
    hodor.insert_form("key", v.to_owned());
    let t = format!("HoldTheDoor={}", v);
    header.insert(
        COOKIE,
        HeaderValue::from_str(&t).expect("Cookie value to be here"),
    );

    hodor.insert_form("holdthedoor", "Submit");
    dbg!(&header);
    dbg!(hodor.get_form());
    let s = client
        .post(&hodor.get_url())
        .form(&hodor.get_form())
        .headers(header);
    dbg!(&s);
    let _ = s.send();
    Ok(())
}

#[cfg(target_os = "windows")]
fn enable_ansi() {
    if cfg!(windows) && ansi_term::enable_ansi_support().is_err() {
        colored::control::set_override(false);
    }
}

#[cfg(target_os = "linux")]
fn enable_ansi() {}

fn run_clap(mut hodor: HodorT) -> HodorT {
    let yml = load_yaml!("../../cli.yml");
    let m = clap::App::from_yaml(yml).get_matches();

    let mut pb = ProgressBar::new(100);
    pb.format("╢▌▌░╟");

    if let Some(id) = m.value_of("id") {
        println!(
            "id = {:?}",
            &id.parse::<u64>().unwrap_or_else(|_| {
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
            a.parse::<u64>().unwrap_or_else(|_| {
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

    hodor
}
