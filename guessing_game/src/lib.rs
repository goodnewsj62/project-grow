pub use std::io;

pub mod helpers{

    use std::io::Write;

    use rand::prelude::*;

    pub fn display_welcome(){
        println!("*****          *****  ********************  ******                  ******************    ****************    ******        ******  ********************");
        println!("******        ******  ********************  ******                ********************  ********************  ********    ********  ********************");
        println!("******* **** *******  ******                ******                ******                ******        ******  *********  *********  ******              ");
        println!("********************  ********************  ******                ******                ******        ******  ********************  ********************");
        println!(" ******************   ********************  ******                ******                ******        ******  ********************  ********************");
        println!(" ***** ****** *****   ******                ******                ******                ******        ******  ********************  ******              ");
        println!("  ****  ****  ****    ********************  ********************  ********************  ********************  ******* ***** ******  ********************");
        println!("  ***          ***    ********************  ********************    ******************    ****************    ********     *******  ********************");
    }

    pub fn generate_random()->u32{
        let mut rng = rand::thread_rng();
        let y: u32 = rng.gen_range(0..10);

        y
    }

    pub fn compare_values(guess:u32,  generated:  u32){
        if guess == generated {
            println!("\nHurray you guessed right, the generated number is {} \n",  guess);
            return;
        }
        
        println!("WRONG CHOICE! The generated number is: {}\n",  generated);
        println!("\nGAME OVER!\n")
    }

    pub fn input(query: &str) -> String {
        print!("\n{}: ",  query);
        let _  =  std::io::stdout().flush();
        let mut input =  String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read line");
        input
    }
}


#[cfg(test)]
mod test{
    
}