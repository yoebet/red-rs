use serde_json::{Value};
use std::fs::File;
use std::io;
use std::io::BufRead;

// struct Entry {
//     word: String,
//     base_form: String,
// }

fn build_base_forms(file_name: &str) {
    //     'word', 'baseForm', 'baseForms', 'examType',
    //         'phonetics', 'pronAllUk', 'pronAllUs', 'simple',
    //         'forms', 'categories', 'phrases'

    let file = File::open(file_name).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    lines.next();
    let it = lines.next();
    let fl = it.unwrap().unwrap();
    println!("fl: {}", fl);
    let v: Value = serde_json::from_str(&fl).unwrap();
    println!("v: {:?}", v);
    if let Value::Object(o) = v {
        println!("word: {:?}", o.get("word").or(None).unwrap());
        println!("word2: {:?}", o.get("word2"));
    }
}

#[test]
fn test_build_froms() {
    build_base_forms(r#"C:\ws\node\cla-data\bson\dict-comp.jsonl"#);
}

fn guest_base_forms(word: &str) -> Vec<String> {
    let mut forms: Vec<String> = Vec::new();
    let len = word.len();
    if word.ends_with("s") {
        if word.ends_with("es") {
            if word.ends_with("ies") {
                forms.push(word[..word.len() - 3].to_owned() + "y");
            } else {
                forms.push(word[..len - 2].to_owned());
            }
        }
        if !word.ends_with("ss") {
            forms.push(word[..len - 1].to_owned());
        }
    }
    println!("{:?}", forms);
    forms
}

#[test]
fn test_base_forms() {
    let _ = guest_base_forms("trees");
    let _ = guest_base_forms("cheeses");
    let _ = guest_base_forms("tomatoes");
    let _ = guest_base_forms("ferries");
}
