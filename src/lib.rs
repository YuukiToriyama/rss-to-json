use std::borrow::Cow;
use hard_xml::XmlRead;

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "feed")]
struct Feed<'a> {
    #[xml(attr = "xmlns")]
    xmlns: Cow<'a, str>,
    #[xml(child = "title")]
    title: Option<Title<'a>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "title")]
struct Title<'a> {
    #[xml(text)]
    text: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_tag() {
        let xml = r#"<feed xmlns="http://www.w3.org/2005/Atom"></feed>"#;
        let parsed_xml = Feed::from_str(xml).unwrap();
        assert_eq!(parsed_xml, Feed { xmlns: "http://www.w3.org/2005/Atom".into(), title: None })
    }

    #[test]
    fn parse_complex_xml() {
        let xml = r#"<feed xmlns="http://www.w3.org/2005/Atom">
            <title>Hogehoge RSS Feed</title>
        </feed>"#;
        let parsed_xml = Feed::from_str(xml).unwrap();
        assert_eq!(parsed_xml, Feed { xmlns: "http://www.w3.org/2005/Atom".into(), title: Some(Title { text: "Hogehoge RSS Feed".into() }) })
    }
}
