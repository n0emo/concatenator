// Concatenator - quickly concatenate many files into one
// Copyright © 2026 Albert Shefner <usn0emo@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Concatenator - quickly concatenate many files into one

// TODO: Support more file extensions
// TODO: Support file extension groups
// TODO: Consider perfect hash map for file extensions and groups
// TODO: Document template syntax
// TODO: Parallelize
// TODO: Exclude pattern

use std::{borrow::Cow, error::Error, fs, path::PathBuf};

use clap::Parser;
use unescape::unescape;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short)]
    dir: Vec<PathBuf>,

    #[arg(long, short)]
    prefix: Option<String>,

    #[arg(long, short)]
    extension: Vec<String>,

    #[arg(long, short)]
    out: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let prefix = match &args.prefix {
        Some(p) => unescape(p),
        None => Some(String::new()),
    };

    let mut result = String::new();
    for path in &args.dir {
        for entry in WalkDir::new(path).sort_by_file_name() {
            let Ok(entry) = entry else {
                continue;
            };

            if entry.file_type().is_dir() {
                continue;
            }

            let filename = entry.file_name().to_string_lossy();

            let ext = entry
                .path()
                .extension()
                .map(|o| o.to_string_lossy())
                .or_else(|| {
                    if let Some(s) = filename.strip_prefix('.') {
                        Some(Cow::Borrowed(s))
                    } else {
                        None
                    }
                })
                .unwrap_or(Cow::Borrowed(""));

            if !args.extension.is_empty() && args.extension.iter().all(|s| *s != *ext) {
                continue;
            }

            if let Some(prefix) = &prefix {
                let prefix = prefix.replace("%{path}", &entry.path().to_string_lossy());
                let prefix = prefix.replace(
                    "%{comment}",
                    match ext.as_ref() {
                        "rs" | "c" | "cpp" | "h" | "hpp" => "//",
                        "toml" | "typ" | "py" | "sh" | "fish" | "yml" | "yaml" | "gitignore" => "#",
                        "lua" | "sql" => "--",
                        _ => "//",
                    },
                );
                result += &prefix;
            }

            let contents = fs::read_to_string(entry.path())?;
            result += &contents;
        }
    }

    match &args.out {
        Some(out) => fs::write(out, result)?,
        None => println!("{result}"),
    }

    Ok(())
}
