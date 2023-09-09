#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut degree = vec![0; num_courses as usize];

        let mut successor = vec![vec![]; num_courses as usize];

        for pre in prerequisites {
            let a = pre[0];
            let b = pre[1];

            degree[a as usize] += 1;
            successor[b as usize].push(a);
        }

        let mut q = std::collections::VecDeque::new();

        for i in 0..num_courses {
            if degree[i as usize] == 0 {
                q.push_back(i);
            }
        }

        while !q.is_empty() {
            let curr = q.pop_front().unwrap();

            for &succ in &successor[curr as usize] {
                degree[succ as usize] -= 1;
                if degree[succ as usize] == 0 {
                    q.push_back(succ);
                }
            }
        }

        degree.iter().all(|&d| d == 0)
    }
}

#[test]
fn it_works() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];

    let res = Solution::can_finish(num_courses, prerequisites);

    assert_eq!(res, true);
}
