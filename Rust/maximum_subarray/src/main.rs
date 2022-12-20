use std::io;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut partsum = 0;
    let mut minpartsum = 0;
    let mut maxres = nums[0];
    for i in 0..nums.len() {
        partsum += nums[i];
        if partsum-minpartsum > maxres {maxres = partsum-minpartsum;}
        if partsum < minpartsum {minpartsum = partsum;}
    }
    maxres
}

fn main() {
    let mut nums = String::new();  io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i32>,_>>().unwrap();
    println!("{}", max_sub_array(nums));
}
