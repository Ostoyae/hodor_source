use reqwest;
use scraper::{Html, Selector};
use std::{collections::HashMap, option::Option, result::*, thread, time::Duration};
use pbr::ProgressBar;
//use colored::*;

/// A Hodor structure
/// Todo: implement url
#[derive(Default, Debug)]
pub struct HodorStruct {
    html: Option<String>,
    votes: HashMap<String, u64>,
    form: HashMap<&'static str, String>,
    url: Option<String>,
    pub goal: u64,
    pub cookies: bool
}

pub type HodorT = HodorStruct;

impl HodorStruct {
    pub fn new() -> HodorStruct {
        HodorStruct::default()
    }

    pub fn set_url(&mut self , url : String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub fn set_goal(&mut self , goal : u64) -> &mut Self {
        if goal > 1024 {self.goal = u64::from(goal)}
        self
    }

    pub fn get_html(&mut self) -> Result<(), reqwest::Error> {
        let url = "http://158.69.76.135/level0.php";
        let html = reqwest::get(url)?.text().unwrap();

        self.html = Some(html);
        Ok(())
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

    pub fn insert_form<S>(&mut self, key: &'static str, value: S) -> &mut Self
        where S: Into<String>
    {
        self.form.insert(key, value.into());
        self
    }

    pub fn post_req(self) -> Result<(), reqwest::Error> {
        let voter: &str = self.form.get("id").expect("Id Value");
        let client = reqwest::Client::new();
        let count: u64 = self.votes.get(voter).expect("voter's current score") - self.goal;
        let mut pb = ProgressBar::new(count);
        pb.format("╢▌▌░╟");
        let handle = thread::spawn(move || {
            for _i in 0..count {
                pb.inc();
                let _req = client.post("http://158.69.76.135/level0.php")
                                 .form(&self.form)
                                 .send()
                                 .expect("this to work");
                thread::sleep(Duration::from_millis(2));
            }
            pb.finish_print("Votes been casted");
        });

        handle.join().expect("handle failed");

        Ok(())
    }

    pub fn fake_post_req(self) -> Result<(), reqwest::Error> {
        let voter: &str = self.form.get("id").expect("Id Value");
        let count: u64 = self.votes.get(voter).expect("voter's current score") - self.goal;

        let mut pb = ProgressBar::new(count);
        pb.format("╢▌▌░╟");
        let handle = thread::spawn(move || {
            for _i in 0..count {
                pb.inc();
                thread::sleep(Duration::from_millis(3))
            }
            pb.finish_print("Votes been casted");
        });

        handle.join().expect("handle failed");

        Ok(())
    }
}
