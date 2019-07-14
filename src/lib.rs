#[macro_use]
extern crate serde_derive;

mod dict;
mod dict_codegen;

pub mod report;
pub mod tokens;

pub use crate::dict::*;

use std::fs::File;
use std::io::Read;

pub fn process_file(
    path: &std::path::Path,
    dictionary: &Dictionary,
    ignore_hex: bool,
    report: report::Report,
) -> Result<(), failure::Error> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    for (line_idx, line) in grep_searcher::LineIter::new(b'\n', &buffer).enumerate() {
        let line_num = line_idx + 1;
        for ident in tokens::Identifier::parse(line) {
            if !ignore_hex && is_hex(ident.token()) {
                continue;
            }
            if let Some(correction) = dictionary.correct_ident(ident) {
                let col_num = ident.offset();
                let msg = report::Message {
                    path,
                    line,
                    line_num,
                    col_num,
                    typo: ident.token(),
                    correction,
                    non_exhaustive: (),
                };
                report(msg);
            }
            for word in ident.split() {
                if let Some(correction) = dictionary.correct_word(word) {
                    let col_num = word.offset();
                    let msg = report::Message {
                        path,
                        line,
                        line_num,
                        col_num,
                        typo: word.token(),
                        correction,
                        non_exhaustive: (),
                    };
                    report(msg);
                }
            }
        }
    }

    Ok(())
}

fn is_hex(ident: &str) -> bool {
    lazy_static::lazy_static! {
        // `_`: number literal separator in Rust and other languages
        static ref HEX: regex::Regex = regex::Regex::new(r#"^0[xX][0-9a-fA-F_]+$"#).unwrap();
    }
    HEX.is_match(ident)
}
