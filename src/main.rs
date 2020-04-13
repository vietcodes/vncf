use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::HashMap;

fn main() {
    use walkdir::WalkDir;

    let mut problems = HashMap::new();
    for entry in WalkDir::new("html") {
        let entry = entry.unwrap();
        if entry.path().extension().and_then(|x| x.to_str()) != Some("html") {
            continue;
        }
        let text = std::fs::read_to_string(entry.path()).unwrap();
        let document = Document::from(text.as_str());

        let table = document.find(Attr("class", "problems")).next().unwrap();
        for link in table.find(Name("a")) {
            let text_owned = link.inner_html();
            let mut text = text_owned.trim();

            if text.starts_with("<!--") {
                text = &text[4..];
                text = text.trim_left();
                assert!(text.starts_with("-->"));
                text = &text[3..];
            }
            if text.ends_with("-->") {
                text = &text[..text.len() - 3];
                text = text.trim_right();
                assert!(text.ends_with("<!--"));
                text = &text[..text.len() - 4]
            }
            if !text.starts_with("<img") && text.len() > 1 {
                let split: Vec<_> = text.splitn(2, "-").collect();
                assert!(split.len() == 2);
                let id = split[0].trim();
                let name = split[1].trim();
                let href = "https://codeforces.com".to_owned() + link.attr("href").unwrap();
                problems.insert(id.to_owned(), (name.to_owned(), href));
            }
        }
    }
    let _ = std::fs::remove_dir_all("output");
    let _ = std::fs::create_dir("output");
    for (id, (title, url)) in problems.iter() {
        let content = format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv = "refresh" content = "0; url = {url}" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{id} - {title}</title>
</head>
<body>
    Redirecting to {url}
</body>
</html>
"#,
            url = url,
            title = title,
            id = id
        );
        std::fs::create_dir(&format!("output/{}", id)).unwrap();
        std::fs::write(&format!("output/{}/index.html", id), &content).unwrap();
    }
}
