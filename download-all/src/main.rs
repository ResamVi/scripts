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

fn main() {
    let re = Regex::new(r".+it/(\w+).+(\.\w+)").unwrap();
    // https://v.redd.it/bfqq6lmiy2091/DASH_720.mp4
    // https://i.redd.it/ex651i5pruy81.jpg
    let text = "https://v.redd.it/bfqq6lmiy2091/DASH_720.mp4";
    let caps = re.captures(text).unwrap();
    println!("{:?}", caps.get(1).unwrap().as_str());
    println!("{:?}", caps.get(2).unwrap().as_str());

    // let response = ureq::get("https://i.redd.it/ex651i5pruy81.jpg")
    //     .call()
    //     .unwrap();

    // println!("{:?}", response.headers_names());

    // let mut file = fs::File::create("test.jpg").unwrap();
    // std::io::copy(&mut response.into_reader(), &mut file).unwrap();


    // let foo: String = fs::read_to_string("mf.har").unwrap();

    // let file: HarFile = serde_json::from_str(&foo).unwrap();

    // for entry in file.log.entries {
    //     let url = entry.request.url;

    //     if !url.contains("redd.it") {
    //         continue;
    //     }

    //     if url.contains("DASH_audio") {
    //         continue;
    //     }

    //     if url.contains("preview") {
    //         continue;
    //     }

    //     println!("{}", url);

    //     let (_, filename) = url.split_once(".it/");

        // let response = ureq::get("http://example.com")
        //     .call()
        //     .unwrap();
    // }
}
