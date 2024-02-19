use std::env;
use std::fs;
use std::path::Path;
//use std::thread::JoinHandle;
use regex::Regex;
use reqwest;
use clap::{command, Arg};
//use std::thread;
// use thread_id;
// use std::sync::{Arc, Mutex};
// use std::rc::Rc;

fn main() {

    let match_result = command!()
    .about("This program accepts a path to a directory and recursively searches through files for broken URLs inside them.")
    .arg(
        Arg::new("directory_clap")
        .help("The directory input")
    )
    .get_matches();

    let args = match_result.get_one::<String>("directory_clap").unwrap();

    let directory_path = Path::new(args);

    if !(directory_path.exists()){
        panic!("Please enter a valid path");
    }

    //Print statement for formatting a table
    println!("{0: <120} | {1: <15} | {2:10}" ,
    "Path", "Status", "Status Code");

    //If the path entered is just a single file, it is sent directly to be read
    if directory_path.is_file(){
        file_reader(args);
        return;
    }

    directory_reader(directory_path);
}

//The directory reader recursively goes through the directory and sends files to file_reader()
fn directory_reader(directory: &Path){
    let paths = fs::read_dir(directory).unwrap();

    for path in paths{
        let path_buf = path.unwrap().path();
        if path_buf.is_dir() {
            //let new_thread = thread::spawn(move ||{
                directory_reader(&path_buf);
            //});
            
        }
        else if path_buf.is_file(){
            file_reader(path_buf.as_os_str().to_str().unwrap());
        }
        else{            
            println!("{} not recognized as a directory or file", path_buf.display());
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
fn file_reader(path: &str){
    let contents = fs::read_to_string(path)
    .expect("Cannot read file");

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
        }
        else {
            status = StatusOptions::ValidURL;
            status_code = response.unwrap().status().to_string();
        }
    }

    //Match to connect enum options to appropriate string slices to be printed
    let status_string = match status {
        StatusOptions::InvalidURL => "Invalid",
        StatusOptions::NoURLs => "None",
        StatusOptions::ValidURL => "Valid"
    };

    //Printing based on every file
    println!("{0: <120} | {1: <15} | {2:10}" ,
    {path}, {status_string}, {status_code});
}