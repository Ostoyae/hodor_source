use reqwest::{Error, Response};
use scraper::{Html, Selector};
use std::collections::HashMap;

fn main() {
    let html = r#"
        <ul>
            <li>Foo</li>
            <li>Bar</li>
            <li>Baz</li>
        </ul>
        "#;

    let fragment = Html::parse_fragment(html);
    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let ul = fragment.select(&ul_selector).next();
    for element in ul.select(&li_selector) {
        println!("{:?}", element.inner_html());
    }
}
