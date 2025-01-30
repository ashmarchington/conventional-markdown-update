#![feature(let_chains)]

use std::fmt::Display;

mod commit;
mod config;
mod writer;

#[derive(Debug)]
enum ActionError<'a> {
    Commit(&'a str),
}

impl Display for ActionError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ActionError::Commit(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl core::error::Error for ActionError<'_> {}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let commits = commit::load_file(args[1].clone()).unwrap();

    let config = match std::env::var("CMU_CONFIG") {
        Ok(v) => match config::load_file(&v) {
            Some(c) => c,
            None => {
                eprintln!("error");

                return;
            }
        },
        Err(e) => {
            eprintln!("error {e}");

            return;
        }
    };

    let path = match std::env::var("CMU_FILE_PATH") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error {e}");

            return;
        }
    };

    let heading = match std::env::var("CMU_AFTER_HEADING") {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error {e}");
            return;
        }
    };

    let level = match std::env::var("CMU_HEADING_LEVEL") {
        Ok(l) => match l.parse::<i32>() {
            Ok(l) => l,
            Err(e) => {
                eprintln!("error {e}");

                return;
            }
        },
        Err(e) => {
            eprintln!("error {e}");
            return;
        }
    };

    let base_url = match std::env::var("CMU_BASE_URL") {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error {e}");
            return;
        }
    };

    match writer::parse_markdown(&base_url, &path, &heading, level, config, commits) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("error {e}");
        }
    }
}
