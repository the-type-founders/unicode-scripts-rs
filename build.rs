use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(serde::Deserialize)]
struct Script {
    code: String,
    name: String,
    ranges: Vec<[u32; 2]>,
}

const DATA: &str = include_str!("vendor/ucd/data.json");

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let root = ok!(std::env::var("OUT_DIR"));
    let path = Path::new(&root).join("data.rs");
    let mut file = ok!(File::create(&path));
    let data: Vec<Script> = ok!(serde_json::from_str(DATA));
    let data = data
        .into_iter()
        .map(|script| {
            let total = script
                .ranges
                .iter()
                .map(|[start, end]| (end - start) as usize)
                .sum::<usize>();
            let ranges = script
                .ranges
                .into_iter()
                .map(|[start, end]| format!("({start},{end})"))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "({:?}, {:?}, {total}, &[{ranges}])",
                script.code, script.name,
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    ok!(write!(
        file,
        "const DATA: &[(&str, &str, usize, &[(u32, u32)])] = &[{data}];",
    ));
}
