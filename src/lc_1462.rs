#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let num_courses = num_courses as usize;
        let mut childs = vec![Vec::<i32>::new(); num_courses];
        let mut pre: Vec<Vec<bool>> = vec![vec![false; num_courses]; num_courses];
        let mut degree = vec![0; num_courses];

        for prerequisite in prerequisites {
            let a = prerequisite[0];
            let b = prerequisite[1];

            degree[b as usize] += 1;
            childs[a as usize].push(b);
        }

        let mut q = std::collections::VecDeque::new();

        for i in 0..num_courses {
            if degree[i] == 0 {
                q.push_back(i);
            }
        }

        while !q.is_empty() {
            let curr = q.pop_front().unwrap();
            for &child in &childs[curr] {
                pre[curr][child as usize] = true;
                for i in 0..num_courses {
                    pre[i][child as usize] = pre[i][child as usize] | pre[i][curr];
                }
                degree[child as usize] -= 1;
                if degree[child as usize] == 0 {
                    q.push_back(child as usize);
                }
            }
        }

        queries
            .into_iter()
            .map(|v| {
                let a = v[0];
                let b = v[1];
                pre[a as usize][b as usize]
            })
            .collect()
    }
}

#[test]
fn it_works() {
    let to_vec = |v: Vec<[i32; 2]>| v.into_iter().map(|a| a.to_vec()).collect();
    let num = 5;
    let pre = [[0, 1], [1, 2], [2, 3], [3, 4]].to_vec();
    let queries = [[0, 4], [4, 0], [1, 3], [3, 0]].to_vec();
    let ans = Solution::check_if_prerequisite(num, to_vec(pre), to_vec(queries));
    assert_eq!(ans, vec![true, false, true, false]);
}
