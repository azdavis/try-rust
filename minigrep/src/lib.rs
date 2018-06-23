use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, corpus: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    for line in corpus.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_one() {
        let query = "They’d banish";
        let corpus = include_str!("../text/poem.txt");
        assert_eq!(
            vec!["They’d banish us, you know."],
            search(query, corpus)
        );
    }
}
