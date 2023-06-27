#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut dp = vec![vec![0; 2]; n];

        let mut max = arr[0];
        dp[0][0] = arr[0];
        dp[0][1] = 0;

        for i in 1..n {
            dp[i][0] = std::cmp::max(dp[i - 1][0] + arr[i], arr[i]);
            dp[i][1] = std::cmp::max(dp[i - 1][0], dp[i - 1][1] + arr[i]);
            max = std::cmp::max(max, std::cmp::max(dp[i][0], dp[i][1]));
        }

        max
    }
}

#[test]
fn it_works() {
    let v = vec![11, -10, -11, 8, 7, -6, 9, 4, 11, 6, 5, 0];
    let res = Solution::maximum_sum(v);
    assert_eq!(res, 50);
}
