extern crate hodor;
extern crate reqwest;
extern crate scraper;

use hodor::HodorT;
use std::result::*;

#[allow(dead_code)]
fn main() -> Result<(), reqwest::Error> {
    let num_votes = 1024;
    let mut hodor = HodorT::new();

//    hodor.set_url("http://158.69.76.135/level0.php");
    hodor.get_html()?;
    hodor.parse_html();
    hodor.insert_form("id", "12345")
        .insert_form("holdthedoor", "Submit+Query");
    hodor.fake_post_req(num_votes)?;

    Ok(())
}
