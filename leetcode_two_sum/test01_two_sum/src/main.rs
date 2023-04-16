use std::collections::HashSet;
use std::convert::TryInto;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut a = HashSet::new();
        let mut count = 0;
        for val in &nums {
            if a.contains(&(target-*val)){
                let loc1 = nums.iter().position(|&r| r == (target-*val)).unwrap();
                return vec![loc1.try_into().unwrap(), count];
            }
            else{
                count += 1;
                a.insert(*val);
            }
        }
        return vec![0];
    }
}
