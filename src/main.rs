use std::env;
use std::process::Command;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
enum SamplerKind {
    #[serde(alias = "ddim", alias = "Ddim")]
    DDIM,
    #[serde(alias = "plms", alias = "Plms")]
    PLMS,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    prompt: String,
    sampler: SamplerKind,
    #[serde(alias = "ckpt")]
    checkpoint: String,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    dbg!(&args);

    let mut map = vec_to_tuple_vec(&args).into_iter().collect::<HashMap<String, String>>();
    for (index, item) in args.iter().enumerate() {
        if index % 2 == 0 {
            map.insert(item[2..].to_string(), args[index + 1][..].to_string());
        }
    }

    let config: Config = toml::from_str(&toml::to_string(&map).unwrap()).unwrap();

    Command::new("python")
        .arg("scripts/txt2img.py")
        .args(["--prompt", config.prompt.as_str()])
        .args(["--ddim_steps=50"])
        .args(["--ckpt", config.checkpoint.as_str()])
        .output()
        .expect("Failed to execute process");


    //dbg!(config);
}

fn vec_to_tuple_vec(vec: &Vec<String>) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for index in 0..vec.len() {
        if index % 2 == 0 {
            result.push((vec[index][..].to_owned(), vec[index + 1][..].to_owned()));
        }
    }
    result
}
