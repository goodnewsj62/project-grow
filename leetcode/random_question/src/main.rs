mod  palindrome_number;
mod special_arrays;
mod longest_prefix;

use palindrome_number::is_palindrome_num;
use longest_prefix::longest_prefix_solutions;

fn main() {
   println!("answer: {}",   is_palindrome_num::first_solution(-121));
   println!("answer: {}",   is_palindrome_num::second_solution(-121));
   
   println!("---------{}",longest_prefix_solutions::solution_one(&vec![String::from("flower"),  String::from("flow"), String::from("flight")]));

   //WRONG!
   println!("answer: {}",   special_arrays::ques_one::get_special_number(vec![0,4,3,0,4]));

}
