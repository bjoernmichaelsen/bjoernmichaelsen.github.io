use std::iter;
use std::fmt::Write;
use std::io::{BufReader, Read};
use std::io::Write as IoWrite;
use std::fs::File;
use xml::reader::{EventReader, XmlEvent};
use std::vec::Vec;
use std::collections::HashMap;


fn format_categories<'a, I>(cats: I) -> String
where I: IntoIterator<Item = &'a str> {
    let cats_w_comma = cats.into_iter()
        .map(|x| {format!(", \"{x}\"")})
        .collect::<String>()
        .clone();
    cats_w_comma
        .split_once(' ')
        .unwrap_or(("",""))
        .1.to_owned()
}

fn article_filename(post_date: &str, post_name: &str) -> String {
    let date_and_time = post_date.split_once(' ').expect("date missing");
    let date = date_and_time.0;
    format!("{0}-{1}.md", date, post_name)
}

fn fix_media_links(content: &str, old_link_prefix: &str, new_link_prefix: &str) -> String {
    content.replace(old_link_prefix, new_link_prefix)
}

fn write_article<W>(mut w: &mut W, post_date: &str, title: &str, link: &str, categories: &str, content: &str)
where W: Write {
    let date_and_time = post_date.split_once(' ').expect("date missing");
    let date = date_and_time.0;
    let title_underline: String = iter::repeat("=").take(title.len()).collect();
    let title_fixed = title.replace("\n", " -- ").replace("\"","");
    writeln!(w, "+++");
    writeln!(w, "title=\"{title_fixed}\"");
    writeln!(w, "date={date}");
    writeln!(w, "[taxonomies]");
    writeln!(w, "originally-published-on=[\"wordpress\"]");
    writeln!(w, "categories=[{categories}]");
    writeln!(w, "+++");
    writeln!(w, "{title}");
    writeln!(w, "{title_underline}");
    writeln!(w, "");
    writeln!(w, "{content}");
    writeln!(w, "Originally published on {post_date} on [wordpress]({link}).");
}

fn parse_wp<R>(r: R, old_link_prefix: &str, new_link_prefix: &str) -> HashMap<String, String>
where R: Read  {
    let buffered = BufReader::new(r); // Buffering is important for performance    
    let parser = EventReader::new(buffered);
    let mut content: String = "".to_owned();
    let mut encoded: String = "".to_owned();
    let mut post_date: String = "".to_owned();
    let mut post_name: String = "".to_owned();
    let mut title: String = "".to_owned();
    let mut link: String = "".to_owned();
    let mut categories = Vec::new();
    let mut articles = HashMap::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                content.clear();
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "post_date" => {
                        post_date = content.clone();
                    }
                    "post_name" => {
                        post_name = content.clone();
                    }
                    "encoded" => {
                        encoded.push_str(content.as_str());
                    }
                    "title" => {
                        title = content.clone();
                    }
                    "link" => {
                        link = content.clone();
                    }
                    "category" => {
                        categories.push(content.replace("\n",""));
                    }
                    "item" => {
                        let article_filename = article_filename(post_date.as_str(), post_name.as_str());
                        let mut article_content = String::new();
                        let cats = format_categories(categories.iter().map(String::as_str));
                        let fixed_link_content = fix_media_links(encoded.as_str(), old_link_prefix, new_link_prefix);
                        write_article(&mut article_content, post_date.as_str(), title.as_str(), link.as_str(), cats.as_str(), fixed_link_content.as_str());
                        if(encoded.len() > 0) {
                            articles.insert(article_filename, article_content);
                        }
                        encoded.clear();
                        categories.clear();
                    }
                    _ => {
                    }
                }
            }
            Ok(XmlEvent::CData(c)) => {
                content.push_str(c.as_str());
            }
            Ok(XmlEvent::Characters(c)) => {
                content.push_str(c.as_str());
            }
            Ok(XmlEvent::Whitespace(c)) => {
                content.push_str(c.as_str());
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }
    articles
}
fn main() {
    let f = File::open("/home/bjoern/checkouts/mirror-skyfromme/youcan039ttaketheskyfromme.wordpress.2023-04-25.000.xml").expect("file should be here.");
    let articles = parse_wp(f, "http://skyfromme.files.wordpress.com/", "/static/img/wp/");
    for (filename, content) in &articles {
            let fullpath = format!("/home/bjoern/checkouts/bjoernmichaelsen.github.io/content/blog/{filename}");
            let mut file = File::create(fullpath).expect("we should be able to create these files");
            write!(file, "{0}", content);
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;    
    use crate::{format_categories, article_filename, parse_wp, fix_media_links};

    #[test]
    fn test_article_filename() {
        let result = article_filename("2011-01-01 23:55:55", "foo-bar-baz");
        assert_eq!(result, "2011-01-01-foo-bar-baz.md");
    }

    #[test]
    fn test_format_categories() {
        let cats = vec!["foo", "bar", "baz"];
        let formatted = format_categories(cats);
        assert_eq!(formatted, "\"foo\", \"bar\", \"baz\"");
    }

    #[test]
    fn test_parse_wp() {
        let f = File::open("/home/bjoern/checkouts/mirror-skyfromme/youcan039ttaketheskyfromme.wordpress.2023-04-25.000.xml").expect("file should be here.");
        let result = parse_wp(f, "http://skyfromme.files.wordpress.com/", "/static/img/wp/");
        assert_eq!(result.len(), 158);
        result.into_keys().for_each(|x| println!("{x}"));
    }

    #[test]
    fn test_fix_media_links() {
        let content = "<em><img class=\"aligncenter wp-image-318\" src=\"http://skyfromme.files.wordpress.com/2012/07/photo.jpg\" alt=\"photo\" width=\"250\" height=\"250\" /></em><p style=\"text-align:center;font-size:x-small;\">photo: (c) 2015 by Bjoern Michaelsen, Creative Commons Attribution-ShareAlike 3.0</p>";
        let result = fix_media_links(content, "http://skyfromme.files.wordpress.com/", "/static/img/wp/");
        assert_eq!(result, "<em><img class=\"aligncenter wp-image-318\" src=\"/static/img/wp/2012/07/photo.jpg\" alt=\"photo\" width=\"250\" height=\"250\" /></em><p style=\"text-align:center;font-size:x-small;\">photo: (c) 2015 by Bjoern Michaelsen, Creative Commons Attribution-ShareAlike 3.0</p>")
    }
}
