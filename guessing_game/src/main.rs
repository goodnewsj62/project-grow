use guessing_game::helpers;


fn main() {
    /*
        print
        WELCOME TO MY GUESSING GAME
        - take a guess like so
        input number guess:
        try and parse the guess
        check if it is a number
        match the if the number equals randomly generated number
        return result
        ask if user wanna play again or quit
        anything other than yes is a quit
        
    */
    helpers::display_welcome();
    let mut should_continue =  true;

    while should_continue {

        println!("\n\nGuess the number GAME!");


        let guess:Result<u32,  _> =  helpers::input("Please input your guess").trim().parse();

        if let Ok(val) =  guess {
            println!("okay {}",  val);

            let generated  =  helpers::generate_random();
            helpers::compare_values(val, generated);
            let resp =   helpers::input("Enter \"yes\" to continue playing");
            should_continue = if resp.trim() == "yes" {true} else {false}
        }else{
            println!("\nEnter a valid number!!!");
            let resp =   helpers::input("Enter \"yes\" to continue playing");
            should_continue = if resp.trim() == "yes" {true} else {false}
        }

    }

    


}
