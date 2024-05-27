mod  palindrome_number;

use palindrome_number::is_palindrome_num;

fn main() {
   println!("answer: {}",   is_palindrome_num::first_solution(-121));
   println!("answer: {}",   is_palindrome_num::second_solution(-121));

}
