use hard_xml::XmlRead;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::structs::Feed;

mod structs;

#[wasm_bindgen]
pub fn parse_rss(xml: &str) -> String {
    let feed = Feed::from_str(xml).unwrap();
    serde_json::to_string(&feed).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_rss() {
        let xml = r#"
            <feed xmlns="http://www.w3.org/2005/Atom">
                <title>Hogehoge RSS Feed</title>
                <updated>2023-09-22T03:07:24Z</updated>
            </feed>"#;
        let json = parse_rss(xml);
        println!("{}", json);
        assert_eq!(
            json,
            r#"{"xmlns":"http://www.w3.org/2005/Atom","title":{"text":"Hogehoge RSS Feed"},"updated_date":{"text":"2023-09-22T03:07:24Z"}}"#
        )
    }
}