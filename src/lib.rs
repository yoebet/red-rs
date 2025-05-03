mod word_form;
mod split_chapters_en;
mod rename_files;

use std::io;
use std::io::BufRead;
use std::io::prelude::*;

type SResult = Result<(), io::Error>;

pub mod ma {
    pub fn la2() {
        println!("la2");
    }
}

