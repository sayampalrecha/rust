use std::collections::HashMap;

impl Solution {
    pub fn_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&prev_index) = seen.get(&complement) {
                return vec![prev_index as i32, i as i32];
            }
            seen.insert(num, i);
        }
        vec![]
    }
}
