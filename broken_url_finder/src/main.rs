use std::env;
use std::fs;
use std::fs::metadata;
use std::path::Path;
use regex::Regex;
use reqwest;

//The main function is responsible for taking in command line arguments and sending them to appropriate methods to be processed
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let directory = &args[1];
    let directory_path = Path::new(directory);

    if !(directory_path.exists()){
        panic!("Please enter a valid path");
    }

    //Print statement for formatting a table
    println!("{0: <120} | {1: <15} | {2:10}" ,
    "Path", "Status", "Status Code");

    //If the path entered is just a single file, it is sent directly to be read
    if directory_path.is_file(){
        file_reader(&directory.to_string());
        return;
    }

    directory_reader(directory_path);
}

//The directory reader recursively goes through the directory and sends files to file_reader()
fn directory_reader(directory: &Path){
    let paths = fs::read_dir(directory).unwrap();

    for path in paths{
        let path_string = path.unwrap().path().to_string_lossy().to_string();
        let md = metadata(&path_string).unwrap();

        //if and else statements that interpret metadata and call either directory_reader recursively or file_reader to be processed

        if md.is_dir(){
            directory_reader(Path::new(&path_string));
        }
        else if md.is_file(){
            file_reader(&path_string);
        }
        else{
            println!("{path_string} not recognized as a directory or file");
        }
    }
}

//enum that helps with the organization of different statuses. It is used in the file_reader() function
enum StatusOptions{
    InvalidURL,
    ValidURL,
    NoURLs,
}

//This function is responsible for reading files and checking for URLs. It then checks if these URLs are valid.
fn file_reader(path: &String){
    let contents = fs::read_to_string(path)
    .expect("Cannot read file: {path}");

    //regex is used to make a guideline for how URLs will be searched for. Specifically looks for items that start with a https
    let url_regex = Regex::new(r#"(https?|ftp)://[^\s/$.?#].[^\s]*"#).unwrap();

    //status is set to NoURLs as a default
    let mut status = StatusOptions::NoURLs;

    let mut status_code = String::from("");

    //loop that goes through each url checking validity
    for url in url_regex.find_iter(&contents) {
        let response = reqwest::blocking::get(url.as_str());

        //if url is invalid it resets status code (unwrapping status code of invalid URL makes the program panic) and breaks out of loop
        if response.is_err() { 
            status = StatusOptions::InvalidURL;
            status_code = String::from("");
            break;
        }
        else {
            status = StatusOptions::ValidURL;
            status_code = response.unwrap().status().to_string();
        }
    }

    //Match to connect enum options to appropriate string slices to be printed
    let status_string = match status {
        StatusOptions::InvalidURL => "Invalid URL",
        StatusOptions::NoURLs => "No URLs",
        StatusOptions::ValidURL => "Valid URL"
    };

    //Printing based on every file
    println!("{0: <120} | {1: <15} | {2:10}" ,
    {path}, {status_string}, {status_code});
}