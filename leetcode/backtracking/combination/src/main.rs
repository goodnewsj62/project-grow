fn main() {
    // here we used backtracking to solve the combination problem (nCr)
    // the algorithm  backtracks once it finds that r == stored value length
    // and leaves any out those that does not meet the criteria

    let intput =  vec![1,2,3,4];

    let mut result =  Vec::new();
    let mut store =  Vec::new();

    let output =  combination(&intput, 3, &mut result, &mut store, 0);

    println!("{:#?}",  output);

}


pub fn combination<'a>(set:  &'a [u8],k: u8 , result:  &'a mut Vec<Vec<u8>>,store: &'a mut Vec<u8>,index: usize)-> &'a Vec<Vec<u8>>{
    let set_size =  set.len();

    if index >=  set_size{
        return result;
    }


    for i  in  (index..set_size).collect::<Vec<usize>>(){
        store.push(set.get(i).unwrap().clone());
        if store.len() ==  usize::from(k){
            result.push(store.clone());
        
        }else{
            combination(set, k, result, store, i + 1);
        }

        store.pop();
    }

    return  result;
}
