use std::fs::File;
use std::{fs, io};
use std::path::{Path, PathBuf};
use crate::SResult;
use std::io::BufRead;
use std::io::prelude::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn split_chapters(path: &str) -> SResult {
    let sp = PathBuf::from(path);
    let fnn = sp.file_name().unwrap().to_str().unwrap();
    let bn = fnn.find("_").unwrap();
    let base_name = &fnn[0..bn];
    let base_dir = sp.parent().unwrap();
    if let Ok(lines) = read_lines(path) {
        let mut current_ch = "".to_string();
        let mut ch_lines: Vec<String> = Vec::new();

        let save_chapter = |content: &String, current_ch: &str| {
            let bnn = base_name.to_owned();
            let np = base_dir.join(bnn + "_" + current_ch + ".txt");
            println!("{:?}", np);
            let mut f = File::create(np).unwrap();
            let _ = f.write_all(content.as_bytes());
        };

        for line in lines.map_while(Result::ok) {
            if line.starts_with("CHAPTER ") {
                let chapter = line.trim_start_matches("CHAPTER ").trim();
                if chapter != current_ch {
                    if current_ch != "" && !ch_lines.is_empty() {
                        save_chapter(&ch_lines.join("\r\n"), &current_ch);
                        ch_lines.clear();
                    }
                    current_ch = chapter.to_string();
                }
            }
            ch_lines.push(line.into());
            // println!("{}", line);
        }
        if !ch_lines.is_empty() {
            save_chapter(&ch_lines.join("\r\n"), &current_ch);
            ch_lines.clear();
        }
    }
    Ok(())
}

#[test]
fn test_fbn() -> SResult {
    // let ss=["aa", "_", "bb", ".txt"].join("\\");
    let path = r"C:\ws\node\voc\wt\chamber\chamber_6-10.txt";
    let sp = PathBuf::from(path);
    let fnn = sp.file_name().unwrap().to_str().unwrap();
    println!("{:?}", fnn);
    let bn = fnn.find("_").unwrap();
    println!("{:?}", &fnn[0..bn]);
    Ok(())
}

#[test]
fn test_split_chapters() -> SResult {
    split_chapters(r"C:\ws\node\voc\wt\chamber\chamber_11-15.txt")
}

#[test]
fn test_split_dir_chapters() -> SResult {
    let dir = r"C:\ws\node\voc\wt\stone";
    let d = Path::new(dir);
    println!("dir {:?}", d.file_name());
    for fe in fs::read_dir(d)?.into_iter() {
        let f = fe?;
        if !f.metadata()?.is_file() {
            continue;
        }
        let path = &f.path();
        let pss = path.to_str().unwrap();
        let hb = pss.find("-");
        if let Some(_) = hb {
            println!("{}", pss);
            let _ = split_chapters(pss);
        }
    }
    Ok(())
}
