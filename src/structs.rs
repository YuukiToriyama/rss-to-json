use std::borrow::Cow;
use hard_xml::XmlRead;

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "feed")]
pub struct Feed<'a> {
    #[xml(attr = "xmlns")]
    xmlns: Cow<'a, str>,
    #[xml(child = "title")]
    title: Option<Title<'a>>,
    #[xml(child = "updated")]
    updated_date: Option<UpdatedDate<'a>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "title")]
struct Title<'a> {
    #[xml(text)]
    text: Cow<'a, str>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "updated")]
struct UpdatedDate<'a> {
    #[xml(text)]
    text: Cow<'a, str>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "entry")]
struct Entry<'a> {
    #[xml(child = "title")]
    title: Title<'a>,
    #[xml(child = "link")]
    link: Link<'a>,
    #[xml(child = "published")]
    published_date: PublishedDate<'a>,
    #[xml(child = "updated")]
    updated_date: UpdatedDate<'a>,
    #[xml(child = "summary")]
    summary: Option<Summary<'a>>,
    #[xml(child = "author")]
    author: Author<'a>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "link")]
struct Link<'a> {
    #[xml(attr = "rel")]
    rel: Cow<'a, str>,
    #[xml(attr = "type")]
    content_type: Cow<'a, str>,
    #[xml(attr = "href")]
    href: Cow<'a, str>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "published")]
struct PublishedDate<'a> {
    #[xml(text)]
    text: Cow<'a, str>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "summary")]
struct Summary<'a> {
    #[xml(text)]
    text: Cow<'a, str>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "author")]
struct Author<'a> {
    #[xml(child = "name")]
    name: Name<'a>,
}


#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "name")]
struct Name<'a> {
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
        assert_eq!(parsed_xml, Feed { xmlns: "http://www.w3.org/2005/Atom".into(), title: None, updated_date: None })
    }

    #[test]
    fn parse_complex_xml() {
        let xml = r#"<feed xmlns="http://www.w3.org/2005/Atom">
            <title>Hogehoge RSS Feed</title>
            <updated>2023-09-22T03:07:24Z</updated>
        </feed>"#;
        let parsed_xml = Feed::from_str(xml).unwrap();
        assert_eq!(
            parsed_xml,
            Feed {
                xmlns: "http://www.w3.org/2005/Atom".into(),
                title: Some(Title { text: "Hogehoge RSS Feed".into() }),
                updated_date: Some(UpdatedDate { text: "2023-09-22T03:07:24Z".into() }),
            })
    }

    #[test]
    fn parse_entry_tag() {
        let xml = r#"
            <entry>
                <title>Today's news</title>
                <link rel="alternate" type="text/html" href="https://example.com/news/20230922.html"/>
                <published>2023-09-22T15:12:39Z</published>
                <updated>2023-09-22T17:07:24Z</updated>
                <summary>It was sunny whole a day.</summary>
                <author>
                    <name>Mr. Hoge</name>
                </author>
            </entry>"#;
        let parsed_xml = Entry::from_str(xml).unwrap();
        assert_eq!(
            parsed_xml,
            Entry {
                title: Title { text: "Today's news".into() },
                link: Link { rel: "alternate".into(), content_type: "text/html".into(), href: "https://example.com/news/20230922.html".into() },
                published_date: PublishedDate { text: "2023-09-22T15:12:39Z".into() },
                updated_date: UpdatedDate { text: "2023-09-22T17:07:24Z".into() },
                summary: Some(Summary { text: "It was sunny whole a day.".into() }),
                author: Author { name: Name { text: "Mr. Hoge".into() } },
            }
        )
    }
}
