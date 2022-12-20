use std::io;

pub fn number_of_ways(n: usize, k: usize, a: Vec<i64>) -> usize {
    let mut sum = 0;
    for i in 0..n {sum += a[i];}
    //easy case
    if sum%(k as i64) != 0 {0}
    //if sum is 0 the solution is just combinatorics
    else if sum == 0 {
        let mut part = 0;
        let mut count = 0; //number of times partial sum is 0
        let mut res = 0; //binomial of count over k-1
        for i in 0..n-1 {
            part += a[i];
            if part == 0 {
                count += 1;
                if count == k-1 {res = 1;}
                if count > k-1 {res = res*count/(count-k+1);} //recursive formula for binomial
            }
        }
        res
    }
    //otherwise we deploy dynamic programming
    else {
        let q = sum/(k as i64);
        let mut part = 0;
        //p[j] is the number of ways we can get j segments with sum q each
        let mut p:Vec<usize> = vec![0; k]; p[0] = 1; 
        for i in 0..n {
            part += a[i];
            if part%q == 0 && part/q > 0 && part/q < (k as i64) {
                //here we deploy the information computed so far
                //if our partial sum is q*j, we have some new ways to get j segments with sum q each
                //each way is made by a division in j-1 segments, plus the remaining segment
                p[(part/q) as usize] += p[(part/q-1) as usize];
            }
        }
        p[k-1]
    }
}

fn main() {
    let mut n = String::new();  io::stdin().read_line(&mut n).unwrap(); 
    let n: usize = n.trim().parse().unwrap();
    let k: usize = 3; //the problem requires k=3, but this solution works for any k 
    let mut a = String::new();  io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i64> = a.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i64>,_>>().unwrap();
    println!("{}", number_of_ways(n, k, a));
}
