// Given an array of integers nums and an integer target, return indices of
// the two numbers such that they add up to target.
// 
// You may assume that each input would have exactly one solution, and you
// may not use the same element twice.
// 
// You can return the answer in any order.
//
// credit: https://leetcode.com/problems/two-sum/description/

use std::collections::HashMap;

/// This version of twosum is quadratic O(n^2) in time complexity. It is
/// very, very slow and should not be done like this other than as a
/// means to explain the concept.
fn slow_twosum(nums: Vec<u32>, target: usize) -> Option<(usize, usize)> {
    for (index, num) in nums.iter().enumerate() {
        for (i, inner_num) in nums.iter().enumerate() {
            // It's important we do not check the current item against
            // itself, as stated in the above prompt.
            if i != index && inner_num + num == target as u32 {
                return Some((index, i));
            }
        }
    }
    None
}

/// This implementation runs in O(n) time complexity as we are only
/// visiting each element in the array once. HashMap operations are O(1)
/// and since we are measuring the average case of the algorithm at
/// scale, we can safely ignore this and estimate that it is O(n).
fn fast_twosum(nums: &[u32], target: u32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target-num)) {
            return Some((i, j));
        }

        // We add the number to the hashmap after viewing it. Thus, we
        // do not need to check that the current item is not being
        // compared against itself.
        map.insert(num, i);
    }
    None
}

fn main() {
    let nums = vec![2,7,11,15];
    let target = 26;
    
    if let Some((i, j)) = fast_twosum(&nums, target) {
        println!("[{i}, {j}]");
    }
    // Sidenote: The problem specifically states we can assume that each input has
    // exactly one solution. Therefore, we do not actually need to
    // return an Option<>.
}
