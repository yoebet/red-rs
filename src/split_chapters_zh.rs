use crate::SResult;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn split_chapters(path: &str) -> SResult {
    let sp = PathBuf::from(path);
    let base_name = "chapter";
    let base_dir = sp.parent().unwrap();
    if let Ok(lines) = crate::read_lines(path) {
        let mut current_ch = "".to_string();
        let mut ch_lines: Vec<String> = Vec::new();

        let save_chapter = |content: &String, current_ch: &str| {
            let bnn = base_name.to_owned();
            let np = base_dir.join(bnn + "_" + current_ch + ".txt");
            println!("{:?}", np);
            let mut f = File::create(np).unwrap();
            let _ = f.write_all(content.as_bytes());
        };
        let re = Regex::new(r"^第([一二三四五六七八九十]{1,3})回\b").unwrap();

        for line in lines.map_while(Result::ok) {
            let m = re.captures(&line);
            if let Some(c) = m {
                let cz = c.get(1).unwrap();
                let cn = zh2n2(cz.as_str());
                let chapter = &cn.to_string()[..];
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
fn test_split_chapters() -> SResult {
    split_chapters(r"C:\ws\node\voc\wt\hlm-rw\红楼梦 舒芜序.txt")
}

#[test]
fn test_match_nums() {
    // let ns="一二三四五六七八九十";
    let re = Regex::new(r"^第([一二三四五六七八九十]{1,3})回\s+").unwrap();
    println!("{:?}", re);
    let m = re.captures("第五十一回 　薛小妹新编怀古诗 胡庸医乱用虎狼药");
    if let Some(c) = m {
        println!("{:?}", c);
        let cz = c.get(1).unwrap();
        println!("{:?}", cz);
        let cn = zh2n2(cz.as_str());
        println!("chapter_{:?}", cn);
    } else {
        println!("{:?}", "...");
    }
}

fn zh2n(zn: &str) -> usize {
    let zs = "0一二三四五六七八九十";
    let mut cs = zn.chars();
    let znl = zn.chars().count();

    let z1 = cs.next().unwrap();
    let (n1, _) = zs.chars().enumerate().find(|(_, c)| *c == z1).unwrap();

    if znl == 1 {
        return n1;
    }
    if znl == 2 {
        if n1 == 10 {
            let z2 = cs.next().unwrap();
            let (n2, _) = zs.chars().enumerate().find(|(_, c)| *c == z2).unwrap();
            return n1 + n2;
        }
        return n1 * 10;
    }
    cs.next();
    let z3 = cs.next().unwrap();
    let (n3, _) = zs.chars().enumerate().find(|(_, c)| *c == z3).unwrap();
    n1 * 10 + n3
}

fn zh2n2(zn: &str) -> i32 {
    let zn_vec: Vec<char> = zn.chars().collect();
    let mut zhm: HashMap<char, i32> = HashMap::new();
    for (i, v) in "一二三四五六七八九十".chars().enumerate() {
        zhm.insert(v, i as i32 + 1);
    }
    let v = zhm.get(&zn_vec[0]).unwrap();
    if zn_vec.len() == 1 {
        return *v;
    }
    if zn_vec.len() == 2 {
        if *v == 10 {
            let v1 = zhm.get(&zn_vec[1]).unwrap();
            return v + v1;
        }
        return v * 10;
    }
    let v2 = zhm.get(&zn_vec[2]).unwrap();
    v * 10 + v2
}

#[test]
fn test_zhh_nums() {
    println!("{:?}", zh2n("五"));
    println!("{:?}", zh2n("五十"));
    println!("{:?}", zh2n("五十一"));
    println!("{:?}", zh2n("十一"));
    println!("{:?}", zh2n2("五"));
    println!("{:?}", zh2n2("五十"));
    println!("{:?}", zh2n2("五十一"));
    println!("{:?}", zh2n2("十一"));
}
