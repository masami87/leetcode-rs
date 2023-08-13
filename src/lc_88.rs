#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;

        let mut idx = m + n - 1;

        while i >= 0 || j >= 0 {
            if i < 0 {
                nums1[idx as usize] = nums2[j as usize];
                j -= 1;
            } else if j < 0 {
                nums1[idx as usize] = nums1[i as usize];
                i -= 1;
            } else if nums1[i as usize] > nums2[j as usize] {
                nums1[idx as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[idx as usize] = nums2[j as usize];
                j -= 1;
            }
            idx -= 1;
        }
    }
}

#[test]
fn it_works() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;
    Solution::merge(&mut nums1, m as i32, &mut nums2, n as i32);

    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6])
}
