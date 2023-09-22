use std::borrow::Cow;
use hard_xml::XmlRead;

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "feed")]
struct Feed<'a> {
    #[xml(attr = "xmlns")]
    xmlns: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_tag() {
        let xml = r#"<feed xmlns="http://www.w3.org/2005/Atom"></feed>"#;
        let parsed_xml = Feed::from_str(xml).unwrap();
        assert_eq!(parsed_xml, Feed { xmlns: "http://www.w3.org/2005/Atom".into() })
    }
}
