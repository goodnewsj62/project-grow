pub mod is_palindrome_num{
    pub fn first_solution(x: i32) -> bool{
        let string = i32::to_string(&x).chars().collect::<Vec<char>>();
        let string_length =  string.len();
    
    
        if string_length == 1{
            return true;
        }
    
    
    
        let mid =  string_length / 2;
        let mut count =  0;
    
        while count < mid{
            let end =  (string_length - 1) - count;
            if string[count].eq(&string[end]){
                count +=  1;
                continue;
            }
    
            return  false;
        }
    
        true
    }


    pub fn second_solution(x:i32)->bool{
        let mut mut_x  =  x;
        let mut x1 =  0;

        if x <  0{
            return  false;
        }

        if x < 10{
            return  true;
        }


        while mut_x != 0{
            x1 *= 10;
            x1 +=  mut_x % 10;
            mut_x /=  10;
        }

        if x1 == x{
            return true;
        }
        
        return false;
    }
}

