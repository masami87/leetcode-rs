#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();

        let mut res = 0;

        let mut gap = std::i32::MAX;

        for i in 0..n - 2 {
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let sum = nums[l] + nums[r] + nums[i];
                if sum == target {
                    return sum;
                }
                if sum < target {
                    if (target - sum).abs() < gap {
                        gap = (target - sum).abs();
                        res = sum;
                    }
                    l += 1;
                } else {
                    if (target - sum).abs() < gap {
                        gap = (target - sum).abs();
                        res = sum;
                    }
                    r -= 1;
                }
            }
        }

        res
    }
}

#[test]
fn it_works() {}
