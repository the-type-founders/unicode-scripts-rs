use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(serde::Deserialize)]
struct Script {
    code: Option<String>,
    name: String,
    ranges: Vec<[u32; 2]>,
}

const DATA: &str = include_str!("vendor/detect-writing-script/data.json");

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let root = ok!(std::env::var("OUT_DIR"));
    let path = Path::new(&root).join("data.rs");
    let mut file = ok!(File::create(&path));
    let data: Vec<Script> = ok!(serde_json::from_str(DATA));
    let data = data
        .into_iter()
        .filter_map(|Script { code, name, ranges }| code.map(|code| (code, name, ranges)))
        .map(|(code, name, mut ranges)| {
            ranges.sort();
            let ranges = ranges
                .into_iter()
                .map(|range| format!("({},{})", range[0], range[1]))
                .collect::<Vec<_>>()
                .join(",");
            format!(r#"("{code}", "{name}", &[{ranges}])"#)
        })
        .collect::<Vec<_>>()
        .join(",");
    ok!(write!(
        file,
        "const DATA: &[(&str, &str, &[(u32, u32)])] = &[{data}];",
    ));
}
