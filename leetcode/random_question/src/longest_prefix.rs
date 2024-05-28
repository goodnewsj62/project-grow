pub mod longest_prefix_solutions{
    pub fn solution_one(x: &Vec<String>)->String{

        let mut y =  x.iter().map(|a| a.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        y.sort_by(|a,b| a.len().cmp(&b.len()));


        let mut result =  String::from("");

        for n in 0..y[0].len(){
            let find =  y[0][n];
            let res =  y.iter().fold( true ,|acc,  cur|  acc && find.eq(&cur[n]) );

            if res{
                result.push(find);
            }
        
        }


        result
    }
}