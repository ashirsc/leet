use std::collections::HashMap;



struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // map value to index of value
        let mut seen_nums = HashMap::<i32,i32>::new();


        for (i,num) in nums.iter().enumerate() {

            // check map for compliment
            let compliment = target - num;
            match seen_nums.get(&compliment) {
                Some(ci) => return vec![*ci, i as i32],
                None => ()
            };

            
            // add to map
            let _ = seen_nums.insert(*num, i as i32);

            
        };

        return vec![]

        
    }
}