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
    let f = if case_sensitive {
        search_cs
    } else {
        search_cis
    };
    f(query, corpus)
}

fn search_cs<'a>(query: &str, corpus: &'a str) -> Vec<&'a str> {
    corpus
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_cis<'a>(query: &str, corpus: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    corpus
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
