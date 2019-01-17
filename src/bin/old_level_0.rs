//extern crate Hodor;
extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use reqwest::{Client, Error};
use std::{collections::HashMap, result::*, thread, time::Duration};

#[allow(dead_code)]
fn main() -> Result<(), reqwest::Error> {

    let num_votes = 1024;

    // Parse the page to find out how many votes exist.
    let html = reqwest::get("http://158.69.76.135/level0.php")?
        .text()
        .unwrap();
    let fragment = Html::parse_fragment(&html);
    let table = Selector::parse("tbody").unwrap();
    let td = Selector::parse("td").unwrap();
    let tbody = fragment.select(&table).next().unwrap();

    // build a HashMap from the parsed data.
    let mut hm_data = HashMap::new();
    for (k, v) in tbody
        .select(&td)
        .skip(2)
        .step_by(2)
        .zip(tbody.select(&td).skip(3).step_by(2))
    {
        hm_data.insert(
            k.inner_html().trim().to_string(),
            v.inner_html().trim().parse::<u32>().unwrap(),
        );
    }

    // config form parms.
    let mut params = HashMap::new();
    params.insert("id", "538");
    params.insert("holdthedoor", "Submit+Query");

    // post votes to server.
    post_req(params, num_votes - hm_data["538"]);

    Ok(())
}

fn post_req(params: HashMap<&'static str, &'static str>, count: u32) {
    let client = reqwest::Client::new();
    let handle = thread::spawn(move || {
        for i in 0..count {
//            print!("{:?} - {:?}", &params, &client);
            let _req = client
                .post("http://158.69.76.135/level0.php")
                .form(&params)
                .send()
                .expect("stuff");
//            thread::sleep(Duration::from_millis(1));
//            println!("post {} of {}", i, count);
        }
    });

    handle.join().expect("threads to work");
}

//    let _res = client
//        .post("http://158.69.76.135/level0.php")
//        .form(&params)
//        .send()
//        .expect("this to work");
//}
