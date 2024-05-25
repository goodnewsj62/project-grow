fn main() {
    let grid =  vec![vec![0,1,2],  vec![3, 4,5],  vec![6,7,8]];
    let mut store: Vec<usize> =  Vec::new();
    find_all_paths(&grid, &mut store, 4, 0, 0);
}


// 0,1,2
// 3,4,5
// 6,7,8
//constraint:  can only move down and side ways


fn find_all_paths(grid:  &Vec<Vec<usize>>  ,store: &mut Vec<usize>, k: u32,column:  usize,  row:  usize){
    if grid.len()  == 0{
        return;
    }


    let row_size =  grid.len();
    let column_size = grid[0].len();


    if row ==  row_size && column ==  column_size{
        return;
    }


    if column >=  row_size{
        return;
    }


    if row>= row_size{
        return;
    }


    if grid[row][column] ==  k as usize{
        let mut diplay_container =store.clone();
        diplay_container.push(grid[row][column].clone());
        print!("------------------------------------");
        println!("path: {:?}", diplay_container );
        return;
    }


    store.push((grid[row][column]).clone());


    find_all_paths(grid,  store, k, column, row + 1);

    find_all_paths(grid,  store, k, column + 1, row);

    store.pop();
    

}