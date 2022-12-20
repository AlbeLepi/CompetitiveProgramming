use std::io;

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    let mut queue: Vec<i32> = Vec::new();
    let mut maxind = 0; for i in 1..nums.len() {if nums[i] > nums[maxind] {maxind = i;}}
    res[maxind] = -1;
    queue.push(nums[maxind]);
    for j in (maxind+1..maxind+nums.len()).rev() {
        let i = j%nums.len();
        while !queue.is_empty() && nums[i] >= *queue.last().unwrap() {queue.pop();}
        res[i] = if queue.is_empty() {-1} else {*queue.last().unwrap()};
        queue.push(nums[i]);
    }
    res
}

fn main() {
    let mut nums = String::new();  io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i32>,_>>().unwrap();
    let res = next_greater_elements(nums);
    for elem in res.iter() {print!("{} ", elem);}
}
