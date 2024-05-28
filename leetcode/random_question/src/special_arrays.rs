pub mod ques_one{
//You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x.

// Notice that x does not have to be an element in nums.

// Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique.

 

// Example 1:

// Input: nums = [3,5]
// Output: 2
// Explanation: There are 2 values (3 and 5) that are greater than or equal to 2.
// Example 2:

// Input: nums = [0,0]
// Output: -1
// Explanation: No numbers fit the criteria for x.
// If x = 0, there should be 0 numbers >= x, but there are 2.
// If x = 1, there should be 1 number >= x, but there are 0.
// If x = 2, there should be 2 numbers >= x, but there are 0.
// x cannot be greater since there are only 2 numbers in nums.
// Example 3:

// Input: nums = [0,4,3,0,4]
// Output: 3
// Explanation: There are 3 values that are greater than or equal to 3.

    pub fn get_special_number(nums:  Vec<i32>) ->  i32{
        let mut nums =  nums;
        nums.sort();

        if nums.len() ==  0 || (nums.len() ==  1 && nums.get(0).unwrap().ne(&1)){
            return  -1 ;
        }


        for n in 1..(nums.last().unwrap().clone()){
            let returned =  search(&nums,n );
            if returned != -1 {
                return returned;
            }
        }
    

        -1
    }


    fn search(nums: & [i32], x: i32) -> i32{
        let  (  mut end,  mut start)=  ( nums.len() -1,0 as usize );

        while start < end{
            let mid = start +  end / 2;

            
            if nums[mid] <= x  && nums[mid + 1] > x{
                if nums[mid..end + 1].len() == x as usize{return  x;}else{ return -1;}
            }
            else if nums[mid] > x{
                end =  mid;
            }else{
                start = mid;
            }
        }

        return  -1;
    }


}