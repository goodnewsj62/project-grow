use std::process;
use std::env;
use text_processing::{validate_args, self};

fn main() {
    //// Simple text processing in rust
    //// this application will take in a path to a file
    //// or a text  from the cli
    //// you must specify the --type followed by f (for file) or t (for text) followed by file path or text
    //// specify what you want to do with the --op flag
    //// w:  word count
    //// c: char count include new line char////
    //// n: line count
    //// r: reverse text
    //// f: word frequency stats
    //// l: find longest word
    //// --mt: to match a text [-i for case insensitive match]
    //// --rt:  replace text [-i  for case insensitive]
    //// --wf:  word frequency for a particular word
    //// EXAMPLE:
    //// cargo run -- --op -wcnfl --mt love --type f /home/user/playground/text.txt

    let mut args: Vec<String> =  env::args().skip(1).collect();

    if let Err(val) = validate_args::perform_args_checks(&args){
        eprint!("{:?}",val);
        process::exit(1);
    }

    text_processing::parse_args(&mut args);


}