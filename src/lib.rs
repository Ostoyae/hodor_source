use reqwest;
use scraper::{Html, Selector};
use std::{collections::HashMap, option::Option, result::*, thread, time::Duration};
use pbr::ProgressBar;

pub struct HodorStruct {
    url: Option<String>,
    html: Option<String>,
    votes: HashMap<String, u32>,
    pub form: HashMap<&'static str, &'static str>,
}

pub type HodorT = HodorStruct;

impl HodorStruct {
    pub fn new() -> HodorStruct {
        HodorStruct {
            url: None,
            html: None,
            votes: HashMap::new(),
            form: HashMap::new(),
        }
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
                    v.inner_html().trim().parse::<u32>().unwrap(),
                );
            }
    }

    pub fn insert_form(&mut self, key: &'static str, value: &'static str) -> &mut Self {
        self.form.insert(key, value);
        self
    }

    pub fn post_req(self, goal: u32) -> Result<(), reqwest::Error> {
        let voter: &str = self.form.get("id").expect("Id Value").as_ref();
        let count: u32 = goal - self.votes.get(voter).expect("voter's current score");
        let client = reqwest::Client::new();
        let mut pb = ProgressBar::new(count as u64);
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
}