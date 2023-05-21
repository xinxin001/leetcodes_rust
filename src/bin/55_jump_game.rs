pub fn can_jump(nums: Vec<i32>) -> bool {
    nums.iter()
        .enumerate()
        .scan(0, |r, (l, l_num)| {
            if *r >= nums.len() - 1 {
                Some(true)
            } else if *r >= l {
                *r = (*r).max(l + *l_num as usize);
                Some(false)
            } else {
                None
            }
        })
        .any(|r| r)
}

//greedy solution
pub fn can_jump1(nums: Vec<i32>) -> bool {
    let mut goal = nums.len() - 1;
    for i in (0..nums.len()).rev() {
        if i + nums[i] as usize >= goal {
            goal = i
        }
    }
    goal == 0
}

//Backtracking solution, will TLE
pub fn backtracking_can_jump(nums: Vec<i32>) -> bool {
    can_jump_from_position(0, &nums)
}

pub fn can_jump_from_position(pos: i32, nums: &[i32]) -> bool {
    if pos == (nums.len() - 1) as i32 {
        return true;
    }
    let furthest_jump = i32::min((nums.len() - 1) as i32, pos + nums[pos as usize]);
    for next_position in (pos + 1)..=furthest_jump {
        if can_jump_from_position(next_position, nums) {
            return true;
        }
    }
    return false;
}

#[derive(Copy, Clone, PartialEq)]
pub enum Index {
    Good,
    Bad,
    Unknown,
}

/*
 * Bottom up solution
 * time: O(n^2)
 * space: O(n)
 */
pub fn bu_can_jump(nums: Vec<i32>) -> bool {
    let mut memo = vec![Index::Unknown; nums.len()];
    let len = memo.len();
    memo[len - 1] = Index::Good;
    for i in (0..len - 1).rev() {
        let furthest_jump = std::cmp::min(i + nums[i] as usize, len - 1);
        for j in (i + 1)..=(furthest_jump as usize) {
            if memo[j] == Index::Good {
                memo[i] = Index::Good;
                break;
            }
        }
    }
    return memo[0] == Index::Good;
}

/*
 * Top down solution
 * time: O(n^2)
 * space: O(n)
 */
pub fn td_can_jump(nums: Vec<i32>) -> bool {
    let mut memo = vec![None; nums.len()];
    memo[nums.len() - 1] = Some(Index::Good);
    td_can_jump_from_position(0, &nums, &mut memo)
}

pub fn td_can_jump_from_position(
    position: usize,
    nums: &Vec<i32>,
    memo: &mut Vec<Option<Index>>,
) -> bool {
    if let Some(memo) = memo[position] {
        return memo == Index::Good;
    }

    let furthest_jump = std::cmp::min(position + nums[position] as usize, nums.len() - 1);
    for next_position in position + 1..=furthest_jump {
        if td_can_jump_from_position(next_position, nums, memo) {
            memo[position] = Some(Index::Good);
            return true;
        }
    }

    memo[position] = Some(Index::Bad);
    return false;
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("{}", backtracking_can_jump(nums));
}
