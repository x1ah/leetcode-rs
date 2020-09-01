use std::collections::HashMap;

struct Solution {}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index: HashMap<i32, i32> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            index.insert(num, idx as i32);
        }

        for (idx, &num) in nums.iter().enumerate() {
            let sub_value = target - num;
            if let Some(&v) = index.get(&sub_value) {
                if v != idx as i32 {
                    return vec![idx as i32, v];
                }
            }
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        target: i32,
        solution: Vec<i32>,
    }
    
    #[test]
    fn tests() {
        let testcases = vec![TestCase{
            input: vec![2, 7, 11, 15],
            target: 9,
            solution: vec![0, 1],
        }, TestCase{
            input: vec![3, 2, 4],
            target: 6,
            solution: vec![1, 2]
        }, TestCase{
            input: vec![3, 3],
            target: 6,
            solution: vec![0, 1]
        }];

        for testcase in testcases.iter() {
            assert_eq!(Solution::two_sum(testcase.input.clone(), testcase.target), testcase.solution);
        }
    }
}