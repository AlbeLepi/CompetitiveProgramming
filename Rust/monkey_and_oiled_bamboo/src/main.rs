use std::io;

fn monkey_and_oiled_bamboo(n: usize, arr: Vec<i32>) -> i32 {
    let mut maxd = arr[0]; let mut k = maxd-1;
    for i in 1..n {
        if maxd < arr[i]-arr[i-1] {maxd = arr[i]-arr[i-1]; k = maxd-1;}
        else if k < arr[i]-arr[i-1] {maxd += 1; k = maxd;}
    }
    maxd
}

fn main() {
    let mut ncases = String::new(); io::stdin().read_line(&mut ncases).unwrap();
    let ncases: usize = ncases.trim().parse().unwrap();
    for i in 0..ncases {
        let mut n = String::new();  io::stdin().read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();
        let mut arr = String::new();   io::stdin().read_line(&mut arr).unwrap();
        let arr: Vec<i32> = arr.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i32>,_>>().unwrap();
        println!("Case {}: {}", i+1, monkey_and_oiled_bamboo(n, arr));
    }
}
