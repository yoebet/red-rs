mod word_form;
mod split_chapters_en;
mod split_chapters_zh;
mod rename_files;

use std::io;
use std::io::BufRead;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

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
