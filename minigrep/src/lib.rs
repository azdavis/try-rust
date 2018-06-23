use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("contents = \n{}", contents);
    Ok(())
}

fn search<'a>(query: &str, corpus: &'a str) -> Vec<&'a str> {
    vec![]
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
