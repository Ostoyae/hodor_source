extern crate Hodor;
extern crate reqwest;
extern crate scraper;
extern crate table_extract;

use scraper::{Html, Selector};
use reqwest::{Client, Error, Response};
use std::{collections::HashMap, result::*, thread, time::Duration};

#[allow(dead_code)]
fn main() -> Result<(), reqwest::Error> {

    let html = reqwest::get("http://158.69.76.135/level0.php")?
        .text()
        .unwrap();
    let fragment = Html::parse_fragment(&html);
    let table = Selector::parse("tbody").unwrap();
    let td = Selector::parse("td").unwrap();
    let tbody = fragment.select(&table).next().unwrap();

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
        //        println!("| {:?} : {:?} |",
        //                 k.inner_html().trim(),
        //                 hm_data.get(k.inner_html().trim()))
    }
    //    println!("{:?}", 1024 - hm_data.get("538").unwrap());
    let mut params = HashMap::new();
    params.insert("id", "538");
    params.insert("holdthedoor", "Submit+Query");

    post_req(params, 1024 - hm_data.get("538").unwrap());

// single post test
//    let mut params = HashMap::new();
//    let params = [("id", 538)];
//    let params2 = ("holdthedoor", "Submit");

//    reqwest::Client::new()
//        .post("http://158.69.76.135/level0.php")
//        .form(&[("id", "12345"), ("holdthedoor", "Submit+Query")])
//        .send()
//        .unwrap();
//    println!("post: {:?}", req);

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
                .expect("this to work");
//            thread::sleep(Duration::from_millis(1));
//            println!("post {} of {}", i, count);
        }
    });

    handle.join().unwrap();
}

//    let _res = client
//        .post("http://158.69.76.135/level0.php")
//        .form(&params)
//        .send()
//        .expect("this to work");
//}
