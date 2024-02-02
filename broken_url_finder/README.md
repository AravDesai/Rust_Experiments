# Program Objectives:
Create a CLI that looks for broken links:
Create a CLI that when you call it looks in the current directory for all files recursively. 
Reads each file and looks for things that look like URLs. When it finds a URL it will test it for validity.

Instructions:
In the command line input:
cargo run [Path to directory]

The output should display the file, whether it has URLs that are valid or not and the Status code of the website accessed if it has a valid URLs

# todo

+ [ ] add CI to your repo. fmt, clippy -- infra
+ [ ] Use clap or cli-rs for clap -- rust specific toolings 
+ [ ] use fewer strings -- type theory learnings
+ [ ] parallize your program -- system programming learnings
