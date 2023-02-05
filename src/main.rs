extern crate unidecode;
extern crate core;

use unidecode::unidecode;

use std::{env, fs, io};
use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;

const ASSETS_TO_LOAD: [&str; 1] = ["words"];
const ASSETS_PREFIX: &str = "assets";

enum Language {
    ptBR,
    enUS,
}

impl Language {
    fn as_str(&self) -> &'static str {
        match self {
            Language::ptBR => "pt-BR",
            Language::enUS => "en-US",
        }
    }

    fn from_str(lang: &str) -> Language {
        match lang {
            "pt-BR" => return Language::ptBR,
            "en-US" => return Language::enUS,
            other => panic!("Language not enabled: {}", other)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lang: Language = if args.len() > 1 {
        Language::from_str(&args[1].as_str())
    } else {
        Language::ptBR
    };

    let assets = load_assets(&lang);

    let mut data: Vec<(String, i32)> = Vec::new();

    loop {
        let mut input= String::new();

        println!("ENTER COMMAND");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.eq("\n") {
            print_options(&assets, &data);
        } else if input.eq("STOP\n") {
            break
        } else {
            let input_parts: Vec<String> = input.split(" ")
                .map(String::from)
                .collect();
            let letter: String = String::from(&input_parts[0]);
            let position : i32 = match input_parts[1].trim().parse() {
                Ok(num) => num,
                Err(err) => break,
            };

            data.push((letter, (position as i32)));
        }
    }
}

fn print_options(assets: &Vec<String>, props: &Vec<(String, i32)>) {
    for word in assets {
        let mut match_props: usize = 0;

        for prop in props {
            if prop.1 < 0 {
                if word.contains(&prop.0)
                    && word.chars().nth(((-1 * prop.1)-1) as usize).unwrap().to_string() != prop.0 {
                    match_props += 1;
                }
            } else if prop.1 > 0 {
                if word.contains(&prop.0)
                    && word.chars().nth((prop.1-1) as usize).unwrap().to_string() == prop.0 {
                    match_props += 1;
                }
            } else {
                if !word.contains(&prop.0) {
                    match_props += 1;
                }
            }
        }

        if match_props == props.len() {
            println!("{}", word);
        }
    }
}

fn load_assets(lang: &Language) -> Vec<String> {
    let mut output: HashMap<String, bool> = HashMap::new();

    for asset  in ASSETS_TO_LOAD {
        for item in load_file(&lang, asset) {
            output.entry(item).or_insert(true);
        }
    }

    return output
        .into_keys()
        .collect();
}

fn load_file(lang: &Language, path: &str) -> Vec<String> {
    let contents  = fs::read_to_string(
        Path::new(ASSETS_PREFIX).join(lang.as_str()).join(path)
    ).expect("Failed to load asset");

    contents
        .split("\n")
        .map(String::from)
        .map(|line| line.to_ascii_lowercase())
        .map(|line| unidecode(line.as_str()))
        .filter(|line| line.len() == 5)
        .collect()
}
