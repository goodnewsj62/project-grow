pub mod validate_args {

    pub fn perform_args_checks(args: &[String]) -> Result<(),String>{
        let validators_ =  [validators::validate_type,  validators::validate_operations,  validators::validate_other_options];

        for func in validators_{
            if let Err(e) = func(args){
                return  Err(e);
            }
        }

        Ok(())
    }



    mod validators {
        use std::path::Path;
        pub fn validate_type(args:  &[String]) -> Result<(),  String>{
            /*
                check if there is a type argument
                check if another non argument come after it
                check if path is valid if type === file
             */


            let pos  = args.iter().position(|val| val.trim().eq("--type"));
            

            if let None =  pos{
                return Err("required param --type not set".to_string());
            }

            let pos =  pos.unwrap_or_default();
            let next_pos =  pos + 1;
            let type_value =  pos +  2;

            let next_pos_value =  args.get(next_pos);
            let type_value =  args.get(type_value);

            if let None =  next_pos_value {
                return Err("f (file) or t(text) must follow --type argument".to_string());
            }

            if let None =  type_value {
                return Err("path to file or text to operate on must follow next".to_string());
            }
            

            if !next_pos_value.unwrap().trim().eq("f") &&  !next_pos_value.unwrap().trim().eq("t"){
                return Err("Invalid file type. only f (file) and t (text) are accepted".to_string());
            }

            if next_pos_value.unwrap().trim().eq("f"){
                if !Path::new(type_value.unwrap().as_str()).is_file(){
                    return Err("File path does not exists or is not a file".to_string());
                }
            }

            Ok(())

        }

        pub fn validate_operations(args:  &[String])-> Result<(),  String>{
            let pos  = args.iter().position(|val| val.trim().eq("--op"));

            if let None =  pos{
                return Err("required param --op not set".to_string());
            }

            let pos =  pos.unwrap_or_default();
            let next_pos =  pos + 1;


            let op_values =  args.get(next_pos);

            if let None =  op_values {
                return Err("an optional value must come after --op".to_string());
            }

            if !op_values.unwrap().starts_with("-"){
                return  Err("optional value must start with a -".to_string());
            }


            for char_  in op_values.unwrap().chars(){
                if char_ == '-'{continue;}

                if !"wcnrfl".contains(char_){
                    return  Err("invalid option option are any combination of w | c | n | r | f | l".to_string());
                }
            }


            Ok(())

        }

        pub fn validate_other_options(args:  &[String])->Result<(), String>{
            let options =  ["--mt",  "--rt", "--wf"];

            for option in options{
                let pos  = args.iter().position(|val| val.trim().eq(option));

                if let None =  pos{
                    continue;
                }

                let follow_text_pos =  pos.unwrap_or_default() + 1;

                if let None =  args.get(follow_text_pos) {
                    return Err(format!("{} requires one positional arg",option));
                }
            }

            Ok(())
        }

    }
    
}

use text_processing_helpers::perform_operations;

pub fn parse_args(input_args:  &mut[String]){

    
    let (mut file_text,  args) =  text_processing_helpers::get_text_string(input_args);
    
    


    for (pos,arg) in args.iter().enumerate(){
       
        match arg.as_str() {
            "--op" =>{
                perform_operations::perform_operations(&mut file_text,  pos,  &args);
            }
            "--mt"=>{
                perform_operations::match_text(&file_text, pos, &args);
            }
            "--rt" =>{
                perform_operations::replace_text(&mut file_text, pos, &args);
            }
            "--wf" =>{
                perform_operations::word_frequency(&file_text, pos, &args)
            }
            _ => {

            }
        }
    }

    print!("\n\nCURRENT TEXT CONTENT \n\n{}\n", file_text)
}


mod text_processing_helpers{
    use std::{fs::File, io::Read, process};
    
    
    pub fn get_text_string(args :  &[String]) -> (String,  Vec<String>){
        let pos =  args.iter().position(|f| f.trim().eq("--type") ).unwrap();

        //remove --type arg and follow ups since we have no use for it
        let transformed_args: Vec<String> =  args.iter().enumerate().filter(|(n, _)| *n != pos && *n != pos + 1 &&  *n != pos + 2).map(|(_, val)| format!("{}", val)).collect();

        // next arg which is f for file and t for text
        if args.get(pos + 1).unwrap().trim().eq("t") {
            return  (args.get(pos + 1).unwrap().clone(), transformed_args);
        }

        let file_ =  File::open(args.get(pos +  2).unwrap());
        
        if let Err(err) = file_  {
            eprint!("Opp could  not read file. this happened: {:?}",  err);
            process::exit(1);
        }

        let mut file_text =  String::new();
        let _  =  file_.unwrap().read_to_string(&mut file_text);
        
        
        (file_text,  transformed_args)

    }


    //// --mt: to match a text [-i for case insensitive match]
    //// --rt:  replace text [-i  for case insensitive]
    //// --wf:  word frequency for a particular word
    pub mod perform_operations {
        use std::collections::HashMap;

        pub fn perform_operations(text: &mut String,pos: usize, args: &Vec<String>){

            for val in args.get(pos +  1).unwrap().chars(){
                if val.to_string().trim().eq("-"){continue;}

                match &val.to_string().trim()[..] {
                    "w" => println!("number of words - {}" , counter(text, "space")),
                    "c" => println!("number of characters - {}" , counter(text, "char")),
                    "n" => println!("number of lines - {}" , counter(text, "line")),
                    "r" => {
                        let _  =  reverse_text(text);
                    },
                    "f"=> println!("word frequency - {:?}" , get_word_frequency(text)),
                    "l"=> println!("longest word found  - {:?}" , find_longest_word(text)),
                    _ => {
                        ()
                    }
                }
            }
        }

        fn counter(string:  &str, delimiter_name: &str ) -> usize{
            let  store  =  HashMap::from([
                ("line",  b'\n'),
                ("space",  b' '),
            ]);


            if !store.contains_key(&delimiter_name){
                    return string.trim().len();
            }

            return string.trim().bytes().fold(1, |acc, e| {
                    if e == *store.get(delimiter_name).unwrap(){
                        return acc +  1;
                    }

                    acc
            });
        }

        fn get_word_frequency(string:  &str) -> HashMap<String,  u32> {
            let mut store  =  HashMap::new();
            string.lines().for_each(|line| {
                line.split(' ').for_each(|key|{*store.entry(key.to_string()).or_insert(1) += 1;});
            });

            store
        }

        fn  find_longest_word (string:  &str) -> String{
            let  (mut max, mut longest_found) =  (0, String::from(""));

            string.trim().split(' ').for_each(|word| {
                if word.len() >  max{
                    max =  word.len();
                    longest_found =  format!("{}",  word);
                }
            });

            longest_found
        }

        fn reverse_text(string:  &mut String) -> &mut String{
            let rev_string: String =  string.chars().rev().collect();
            string.clear();
            string.push_str(&rev_string);
            string
        }

        pub fn match_text(string:  &str, pos: usize,  args: &Vec<String>){
            for (n, line) in string.lines().enumerate() {
                if line.trim().contains(args.get(pos + 1).unwrap()){
                    println!("{}: {}",n+1,line);
                }
            }
        }

        pub fn replace_text(string:  &mut String, pos: usize, args: &Vec<String>){
            let replaced =    string.replace(string.as_str(), args.get(pos + 1).unwrap());
            string.clear();
            string.push_str(&replaced);

        }

        pub fn word_frequency(string:  &str, pos: usize, args: &Vec<String>){
            let result_ =  get_word_frequency(string);
            let word =  args.get(pos + 1).unwrap();
            println!("Frequency of the word {} in text - {}",word,  result_.get(word).unwrap_or(&0));
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn it_works() {
//     //     let result = add(2, 2);
//     //     assert_eq!(result, 4);
//     // }
// }
