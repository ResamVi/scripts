use std::fs::{self, File};
use std::path::Path;
use regex::Regex;

use serde::{Deserialize, Serialize};
use ureq::get;

#[derive(Serialize, Deserialize, Debug)]
struct HarFile {
    log: Log,
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    request: Request,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    url: String
}
//
// https://v.redd.it/bfqq6lmiy2091/DASH_720.mp4
// https://i.redd.it/ex651i5pruy81.jpg

fn main() {
    // Regex pattern for extracting file name.
    let re = Regex::new(r".+it/(\w+).+(\.\w+)").unwrap();

    // Read our archive of requests when we looked through our stuff.
    let foo: String = fs::read_to_string("mf.har").unwrap();

    // Iterate.
    let file: HarFile = serde_json::from_str(&foo).unwrap();
    for entry in file.log.entries {
        let url = entry.request.url;

        if !url.contains("redd.it") {
            continue;
        }

        if url.contains("DASH_audio") {
            continue;
        }

        if url.contains("preview") {
            continue;
        }

        println!("{}", url);

        let caps = re.captures(&url).unwrap();

        let filename = caps.get(1).unwrap().as_str();
        let filetype = caps.get(2).unwrap().as_str();

        let fullname = format!("{}{}", filename, filetype);

        let response = ureq::get(url.as_str())
            .call();

        let response = match response {
            Ok(r) => r,
            Err(_) => continue
        };

        let mut file = fs::File::create(fullname).unwrap();
        std::io::copy(&mut response.into_reader(), &mut file).unwrap();
    }
}
