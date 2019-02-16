use pbr::ProgressBar;
use reqwest;
use reqwest::header::*;
use scraper::{Html, Selector};
use std::{collections::HashMap, option::Option, result::*, sync::mpsc, thread, time::Duration};
use tokio;
//use colored::*;

/// A Hodor structure
/// Todo: implement url
#[derive(Default, Debug, Clone)]
pub struct HodorStruct {
    html: Option<String>,
    votes: HashMap<String, u64>,
    form: HashMap<&'static str, String>,
    url: String,
    goal: u64,
    pub cookies: bool,
}

pub type HodorT = HodorStruct;

impl HodorStruct {
    pub fn new() -> HodorStruct {
        HodorStruct::default()
    }

    pub fn get_url(&self) -> String { self.url.to_owned() }

    pub fn set_url<S>(&mut self, url: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.url = url.into();
        self
    }

    pub fn set_goal(&mut self, goal: u64) -> &mut Self {
        self.goal = goal;
        self
    }


    pub fn get_goal(&self) -> u64 {
        self.goal
    }

    pub fn get_html(&mut self) -> Result<(), reqwest::Error> {
        let url = self.url.clone();
        let html = reqwest::get(&url)?.text().unwrap();

        self.html = Some(html);
        Ok(())
    }

    pub fn get_cookie(&self, client: &reqwest::Client) -> HashMap<String, String> {
        let url = self.url.clone();
        let cookie = client
            .head(&url)
            .header(COOKIE, "HoldTheDoor")
            .send()
            .unwrap()
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_owned();
        let str: String = cookie.to_str().expect("valid str").into();
        let col = str
            .split(";")
            .map(|kv| kv.split("="))
            .map(|mut kv| (kv.next().unwrap().into(), kv.next().unwrap().into()))
            .collect::<HashMap<String, String>>();
        col
    }

    pub fn parse_html(&mut self) {
        let fragment = Html::parse_fragment(&self.html.clone().unwrap());
        let table = Selector::parse("tbody").unwrap();
        let td = Selector::parse("td").unwrap();
        let tbody = fragment.select(&table).next().unwrap();

        // build a HashMap from the parsed data.
        //            let mut hm_data = HashMap::new();
        for (k, v) in tbody
            .select(&td)
            .skip(2)
            .step_by(2)
            .zip(tbody.select(&td).skip(3).step_by(2))
        {
            self.votes.insert(
                k.inner_html().trim().to_string(),
                v.inner_html().trim().parse::<u64>().unwrap(),
            );
        }
    }

    pub fn get_form(&self) -> HashMap<&'static str, String> {self.form.to_owned()}

    pub fn insert_form<S>(&mut self, key: &'static str, value: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.form.insert(key, value.into());
        self
    }

    pub fn post_req(self) -> Result<(), reqwest::Error> {
        let (tx, rx) = mpsc::channel();
        let voter: &str = self.form.get("id").expect("Id Value");
        let count: u64 = self.goal - self.votes.get(voter).expect("voter's current score");
        let mut pb = ProgressBar::new(count);
        pb.format("╢▌▌░╟");

        for i in 0..count {
			let mut ck = false;
			if self.cookies { ck = true }; 
            let tx = tx.clone();
            
            let client = reqwest::Client::new();
            let mut form = self.form.clone();
            let mut header = HeaderMap::new();
            header.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:64.0)"));
            
            let post = client.post(&self.url);
			let url = self.url.to_owned();
            let _handle = thread::spawn(move || {
			
				if ck {
                let mut v;
                let temp = client.get(&url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:64.0)").send();
//                println!("{:?}", &temp);
                let cookie = temp.unwrap().headers().to_owned().get("set-cookie").unwrap().to_owned();
                let str: String = cookie.to_str().expect("valid str").into();
                let col = str
                    .split(";")
                    .map(|kv| kv.split("="))
                    .map(|mut kv| (kv.next().unwrap().into(), kv.next().unwrap().into()))
                    .collect::<HashMap<String, String>>();
                v = col["HoldTheDoor"].to_owned();
                form.insert("key", v.to_owned());
                let t = format!("HoldTheDoor={}", v.to_owned());
                header.insert(COOKIE, HeaderValue::from_str(&t).unwrap());
				}
			
                let _req = post.headers(header).form(&form).send();
                tx.send(i).is_ok();
                thread::sleep(Duration::from_millis(10));
            });
        }

        for _i in 0..count {
            rx.recv().is_ok();
			pb.inc();
        }

        pb.finish_print("Votes been casted");

        Ok(())
    }

    pub fn fake_post_req(self) -> Result<(), reqwest::Error> {
        let voter: &str = self.form.get("id").expect("Id Value");
        let count: u64 = self.get_goal() - self.votes.get(voter).expect("voter's current score");
        let mut pb = ProgressBar::new(count);

        let (tx, rx) = mpsc::channel();

        pb.format("╢▌▌░╟");

        for i in 0..count {
            let tx = tx.clone();
            let mut _form = self.form.clone();
            let _client = reqwest::Client::new();
            if self.cookies {
                //                    let mut v;
                //                    v = self.get_cookie(client)["HoldTheDoor"].to_owned();
                //                    form.insert("HoldTheDoor", v.to_owned());
            }
            let _handle = thread::spawn(move || {
                tx.send(i).unwrap();
            });
            pb.inc();
            //                handle.join().expect("handle failed");
        }

        for _i in 0..count {
            rx.recv().is_ok();
        }
        pb.finish_print("Votes been casted");

        Ok(())
    }
}
