use hard_xml::XmlRead;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::structs::Feed;

mod structs;

#[wasm_bindgen]
pub fn parse_rss(xml: &str, pretty_printed: bool) -> String {
    console_error_panic_hook::set_once();
    let feed = Feed::from_str(xml).unwrap();
    if pretty_printed == true {
        serde_json::to_string_pretty(&feed).unwrap()
    } else {
        serde_json::to_string(&feed).unwrap()
    }
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
        let json = parse_rss(xml, false);
        assert_eq!(
            json,
            r#"{"xmlns":"http://www.w3.org/2005/Atom","title":"Hogehoge RSS Feed","updated_date":"2023-09-22T03:07:24Z","entries":[]}"#
        )
    }

    #[test]
    fn parse_complex_rss() {
        let xml = r#"
            <feed xmlns="http://www.w3.org/2005/Atom">
                <title>Hogehoge RSS Feed</title>
                <updated>2023-09-22T03:07:24Z</updated>
                <entry>
                    <title>Today's news</title>
                    <link rel="alternate" type="text/html" href="https://example.com/news/20230922.html"/>
                    <published>2023-09-22T15:12:39Z</published>
                    <updated>2023-09-22T17:07:24Z</updated>
                    <summary>It was sunny whole a day.</summary>
                    <author>
                        <name>Mr. Hoge</name>
                    </author>
                </entry>
            </feed>"#;
        let json = parse_rss(xml, false);
        assert_eq!(
            json,
            r#"{"xmlns":"http://www.w3.org/2005/Atom","title":"Hogehoge RSS Feed","updated_date":"2023-09-22T03:07:24Z","entries":[{"title":"Today's news","link":{"rel":"alternate","content_type":"text/html","href":"https://example.com/news/20230922.html"},"published_date":"2023-09-22T15:12:39Z","updated_date":"2023-09-22T17:07:24Z","summary":"It was sunny whole a day.","author":{"name":"Mr. Hoge"},"category_list":[]}]}"#
        )
    }
}