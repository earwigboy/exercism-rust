use std::{fs::File, io::BufRead, io::BufReader};

use anyhow::Error;

#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    file_names: bool,
    case_insensitive: bool,
    invert: bool,
    match_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            line_numbers: flags.contains(&"-n"),
            file_names: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            match_lines: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results: Vec<String> = vec![];
    for file in files {
        let f = File::open(file)?;
        let reader = BufReader::new(&f);
        for (index, line) in reader.lines().enumerate() {
            let mut l = line?;
            let result_line = l.to_owned();
            let mut pat = String::from(pattern);
            if flags.case_insensitive {
                l = l.to_lowercase();
                pat = pat.to_lowercase();
            }
            let found_match = if flags.match_lines {
                l == pat
            } else {
                l.contains(&pat)
            };
            if found_match ^ flags.invert {
                if flags.file_names {
                    results.push(String::from(*file));
                    break;
                }
                if flags.line_numbers {
                    if files.len() > 1 {
                        results.push(format!("{}:{}:{}", *file, index + 1, &result_line));
                        continue;
                    }
                    results.push(format!("{}:{}", index + 1, &result_line));
                    continue;
                }
                if files.len() > 1 {
                    results.push(format!("{}:{}", *file, &result_line));
                    continue;
                }
                results.push(format!("{}", &result_line));
            }
        }
    }
    Ok(results)
}
