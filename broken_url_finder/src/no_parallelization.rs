pub mod no_parallel{
use std::path::PathBuf;
use regex::Regex;
use std::fs;
pub fn directory_reader(directory: PathBuf) {
    let paths = fs::read_dir(directory).unwrap();

    for path in paths {
        let path_buf = path.unwrap().path();
        if path_buf.is_dir() {
            directory_reader(path_buf);
        } else if path_buf.is_file() {
            file_reader(path_buf);
        } else {
            println!(
                "{} not recognized as a directory or file",
                path_buf.display()
            );
        }
    }
}

//enum that helps with the organization of different statuses. It is used in the file_reader() function
enum StatusOptions {
    InvalidURL,
    ValidURL,
    NoURLs,
}

//This function is responsible for reading files and checking for URLs. It then checks if these URLs are valid.
pub fn file_reader(path: PathBuf) {
    let contents = fs::read_to_string(path.clone()).expect("Cannot read file");

    //regex is used to make a guideline for how URLs will be searched for. Specifically looks for items that start with a https
    let url_regex = Regex::new(r#"(https?|ftp)://[^\s/$.?#].[^\s]*"#).unwrap();

    //status is set to NoURLs as a default
    let mut status = StatusOptions::NoURLs;

    let mut status_code = String::from("None");

    //loop that goes through each url checking validity
    for url in url_regex.find_iter(&contents) {
        let response = reqwest::blocking::get(url.as_str());

        //if url is invalid it resets status code (unwrapping status code of invalid URL makes the program panic) and breaks out of loop
        if response.is_err() {
            status = StatusOptions::InvalidURL;
            status_code = String::from("Invalid");
            break;
        } else {
            status = StatusOptions::ValidURL;
            status_code = response.unwrap().status().to_string();
        }
    }

    //Match to connect enum options to appropriate string slices to be printed
    let status_string = match status {
        StatusOptions::InvalidURL => "Invalid",
        StatusOptions::NoURLs => "None",
        StatusOptions::ValidURL => "Valid",
    };

    //The following is commented out to make cargo bench look cleaner

    // println!(
    //     "{0: <120} | {1: <15} | {2:10}",
    //     path.display(),
    //     status_string,
    //     status_code
    // );
}}