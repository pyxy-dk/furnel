//! A command-line utility to compress files using the brotli algorithm.

/*************************************************************************************************
 * Copyright © 2022-2025 Kristian Thy (<thy@42.dk>) and released under the MIT license.          *
 * This file is part of Furnel: <https://github.com/pyxy-dk/furnel>                              *
 *************************************************************************************************/

use std::env;
use std::ffi::OsString;
use std::fs::{read, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;

use brotli::CompressorWriter;
use clap::Parser;

use crate::statics::{ABOUT, AFTER_HELP, GLOPTS, LICENSES};

mod statics;

/// Command-line options.
#[derive(Debug, Parser)]
#[clap(about = ABOUT, after_help = AFTER_HELP, term_width = 80)]
pub struct Args {
    /// The base path to search
    #[clap(default_value = ".", required_unless_present = "license")]
    pub base_path: String,
    /// File extensions to process, for example `-x css -x html -x js`
    #[clap(short = 'x', long = "extension", value_name = "EXTENSION", multiple_occurrences = true, number_of_values = 1, default_values = &vec!["css", "html", "js", "svg", "txt"])]
    pub extensions: Vec<String>,
    /// Display full license notice
    #[clap(short, long)]
    pub license: bool,
    /// Only compress missing files, i.e. those where no corresponding .br files are present
    #[clap(short = 'm', long)]
    pub only_missing: bool,
    /// Disable progress indicator
    #[clap(short, long)]
    pub quiet: bool,
    /// Recurse into subdirs below the base path
    #[clap(short, long)]
    pub recurse: bool,
}

fn main() {
    // Load command line arguments
    let args = Args::parse();

    // Short circuit for license display
    if args.license {
        println!("{}", LICENSES);
        exit(0)
    }

    // Infer current working directory
    let cwd = format!(
        "{}",
        env::current_dir()
            .expect("Unable to retrieve current directory!")
            .as_path()
            .display()
    );

    validate_input(&cwd, &args.base_path);

    bake_files(&args);

    if !args.quiet {
        println!(" done!");
    }
}

fn append_br(path: &Path) -> PathBuf {
    let mut os_string: OsString = path.into();
    os_string.push(".br");
    os_string.into()
}

fn bake_file(path: &Path) {
    let input_bytes: Vec<u8> =
        read(path).unwrap_or_else(|_| panic!("error reading {}", path.display()));
    let mut output_file = File::create(append_br(path)).unwrap();
    let mut writer = CompressorWriter::new(&mut output_file, 4096, 11, 21);
    writer
        .write_all(&input_bytes)
        .expect("couldn't bake brotli file!");
    let _ = writer.flush();
}

fn bake_files(args: &Args) {
    let gpat = glob_pattern(&args.base_path, args.recurse);
    // let mut rng = thread_rng();
    for extension in args.extensions.clone() {
        let pattern = &(gpat.to_owned() + &extension);
        let entries = glob::glob_with(pattern, GLOPTS);
        match entries {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(path) => {
                            if !args.only_missing || !append_br(&path).exists() {
                                if !args.quiet {
                                    print!(".");
                                }
                                bake_file(&path)
                            }
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Input error: {:?} is not a valid pattern ({})", gpat, e.msg);
                exit(64)
            }
        }
    }
}

/// Create a glob pattern.
fn glob_pattern(base_path: &str, recurse: bool) -> String {
    format!(
        "{}{}/*.",
        base_path
            .replace('\\', "/")
            .trim_start_matches("./")
            .trim_end_matches('/'),
        if recurse { "/**" } else { "" }
    )
}

/// Exit if `path` is not an existing directory.
fn validate_input(cwd: &str, path: &str) {
    if !Path::new(path).is_dir() {
        eprintln!(
            "Input error: {:?} is not a directory I can read from {}",
            path, cwd
        );
        exit(64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_br() {
        let mut expected = Path::new(".br");
        let mut actual = append_br(Path::new(""));
        assert_eq!(actual, expected);

        expected = Path::new("test.br");
        actual = append_br(Path::new("test"));
        assert_eq!(actual, expected);

        expected = Path::new("test.ext.br");
        actual = append_br(Path::new("test.ext"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_glob_pattern() {
        let mut expected = "foo/*.";
        let mut actual = glob_pattern("foo", false);
        assert_eq!(actual, expected);

        expected = "foo/**/*.";
        actual = glob_pattern("foo", true);
        assert_eq!(actual, expected);

        expected = "tests/files/**/*.";
        actual = glob_pattern(".\\tests\\files\\", true);
        assert_eq!(actual, expected);
    }
}
