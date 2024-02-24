use colored::*;
use std::error::Error;
use std::fs;
use aho_corasick::AhoCorasick;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    search(&config.query, &contents, config.ignore_case);

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            println!("{}", "not enough arguments".red());
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = args.len() > 3 && args[3] == "-i";

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) {
    let ac = AhoCorasick::builder()
                .ascii_case_insensitive(ignore_case)
                .build(&[query])
                .unwrap();

    for line in contents.lines() {

        let mut matches = ac.find_iter(line)
            .fold(vec![], |mut v, m| {v.push(m.start()); v.push(m.end()); v});
        
        if matches.len() == 0 {
            continue;
        }
        
        matches.insert(0, 0);
        matches.push(line.len());

        println!("{:?}", matches[..]
            .windows(2)
            .fold((String::new(), true), |(str, f), x| {
                (format!("{str}{:?}", if f {line[x[0]..x[1]].white()} else {line[x[0]..x[1]].red()}), !f)
            })
        );

        // let new_line = if ignore_case {
        //     line.to_lowercase()
        // } else {
        //     line.to_owned()
        // };

        // if new_line.contains(query) {
        //     let new_line = new_line.split(query).collect::<Vec<&str>>();

        //     let last_para = new_line[new_line.len() - 1];

        //     println!(
        //         "{}{}",
        //         new_line[..new_line.len() - 1]
        //             .into_iter()
        //             .fold(String::new(), |str, x| format!("{str}{x}{}", query.red())),
        //         last_para
        //     );
        // }
    }
}


