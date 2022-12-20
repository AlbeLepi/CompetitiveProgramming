use std::io;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut peek = 0; 
    for i in 1..height.len() {if height[i] > height[peek] {peek = i;}}
    let mut water = 0;
    let mut heightpar = height[0];
    for i in 1..peek {
        if height[i] > heightpar {heightpar = height[i];}
        else {water += heightpar-height[i];}
    }
    heightpar = height[height.len()-1];
    for i in (peek..height.len()).rev() {
        if height[i] > heightpar {heightpar = height[i];}
        else {water += heightpar-height[i];}
    }
    water
}

fn main() {
    let mut height = String::new();  io::stdin().read_line(&mut height).unwrap();
    let height: Vec<i32> = height.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i32>,_>>().unwrap();
    println!("{}", trap(height));
}
