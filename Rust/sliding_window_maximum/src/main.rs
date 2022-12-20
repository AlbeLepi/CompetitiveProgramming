use std::io;
use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut res = Vec::with_capacity(nums.len()-k+1);
    let mut deque: VecDeque<(i32,usize)> = VecDeque::new();
    for i in 0..nums.len() {
        while !deque.is_empty() && deque.back().unwrap().0 <= nums[i] {deque.pop_back();}
        deque.push_back((nums[i],i));
        if k+deque.front().unwrap().1 <= i {deque.pop_front();}
        if k-1 <= i {res.push(deque.front().unwrap().0);}
    }
    res
}

fn main() {
    let mut k = String::new();  io::stdin().read_line(&mut k).unwrap(); 
    let k: i32 = k.trim().parse().unwrap(); 
    let mut nums = String::new();  io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i32>,_>>().unwrap();
    let res = max_sliding_window(nums, k);
    for i in 0..res.len() {print!("{} ", res[i]);}
}
