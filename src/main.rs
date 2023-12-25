use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let path = &args[2];
    if let Ok(ret) = get_csv(&String::from(url)) {
        let _ = write_csv(&ret, &String::from(path));
    }
}

fn get_csv(link: &String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(link)?.text();
    return Ok(resp?);
}

fn write_csv(value: &String, dest: &String) -> std::io::Result<()> {
    let mut file = File::create(dest)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}
