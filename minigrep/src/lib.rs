use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for line in search(
        &config.query,
        &contents,
        config.case_sensitive
    ) {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(
    query: &str,
    corpus: &'a str,
    case_sensitive: bool
) -> Vec<&'a str> {
    if case_sensitive {
        search_cs(query, corpus)
    } else {
        search_cis(query, corpus)
    }
}

fn search_cs<'a>(query: &str, corpus: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for line in corpus.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    }
    ret
}

fn search_cis<'a>(query: &str, corpus: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    let query = query.to_lowercase();
    for line in corpus.lines() {
        if line.to_lowercase().contains(&query) {
            ret.push(line);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "They’d banish";
        let corpus = include_str!("../text/poem.txt");
        let case_sensitive = true;
        assert_eq!(
            vec!["They’d banish us, you know."],
            search(query, corpus, case_sensitive),
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RUST";
        let corpus = include_str!("../text/rust.txt");
        let case_sensitive = false;
        assert_eq!(
            vec![
                "Rust: a programming language.",
                "Trust me.",
            ],
            search(query, corpus, case_sensitive)
        )
    }
}
