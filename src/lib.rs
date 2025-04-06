use std::fs::{self, File};
use std::io;
use std::io::BufRead;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

type SResult = Result<(), io::Error>;

pub mod ma {
    pub fn la2() {
        println!("la2");
    }
}

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

        // let mut save_chapter = || {
        //     let bnn = base_name.to_owned();
        //     let np = base_dir.join(bnn + "_" + current_ch + ".txt");
        //     println!("{:?}", np);
        //     let mut f = File::create(np).unwrap();
        //     let content = ch_lines.join("\r\n");
        //     let _ = f.write_all(&content.as_bytes());
        //     ch_lines.clear();
        // };

        for line in lines.map_while(Result::ok) {
            if line.starts_with("Chapter ") {
                let chapter = line.trim_start_matches("Chapter ").trim();
                if chapter != current_ch {
                    if current_ch != "" && !ch_lines.is_empty() {
                        let bnn = base_name.to_owned();
                        let np = base_dir.join(bnn + "_" + &current_ch + ".txt");
                        println!("{:?}", np);
                        let mut f = File::create(np).unwrap();
                        let content = ch_lines.join("\r\n");
                        let _ = f.write_all(&content.as_bytes());
                        ch_lines.clear();
                    }
                    current_ch = chapter.to_string();
                }
            }
            ch_lines.push(line.into());
            // println!("{}", line);
        }
        if !ch_lines.is_empty() {
            let bnn = base_name.to_owned();
            let np = base_dir.join(bnn + "_" + &current_ch + ".txt");
            println!("{:?}", np);
            let mut f = File::create(np).unwrap();
            let content = ch_lines.join("\r\n");
            let _ = f.write_all(&content.as_bytes());
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
    split_chapters(r"C:\ws\node\voc\wt\chamber\chamber_6-10.txt")
}

fn pdir(d: &Path) {
    println!("dir {:?}", d.file_name());
    for fe in fs::read_dir(d).unwrap().into_iter() {
        let f = fe.unwrap();
        let path = &f.path();
        if f.metadata().unwrap().is_file() {
            pfile(path);
        } else {
            pdir(path);
        }
    }
}

fn pfile(f: &Path) {
    let ns = f.to_str().unwrap();
    if ns.ends_with(".js") {
        let nn = ns.replace(".js", ".ts");
        println!("rename {:?} -> {:?}", ns, nn);
        // fs::rename(ns, nn).unwrap();
    }
}

#[test]
fn test_rename() {
    let d = Path::new(r"C:\ws\node\classic\src\routes");
    pdir(d);
}
