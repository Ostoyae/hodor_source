pub mod hodor {
    use reqwest::{Client, Response, Error};

}

#[cfg(test)]
mod tests {
    use scraper::{Html, Selector};
    use reqwest::{Response,Error};
    use std::collections::HashMap;

    #[test]
    fn first_try(){

            let html = r#"
        <ul>
            <li>Foo</li>cargo
            <li>Bar</li>
            <li>Baz</li>
        </ul>
        "#;

        let fragment = Html::parse_fragment(html);
        let ul_selector = Selector::parse("ul").unwrap();
        let li_selector = Selector::parse("li").unwrap();

        let ul = fragment.select(&ul_selector).next().unwrap();
        for element in ul.select(&li_selector) {
//            assert_eq!("li", element.value().name());
            println!("{:?}", element);
        }

        // let mut params = HashMap::new();
        // params.insert("id", 538 as i32);

        // post_req(params)
        }
}
