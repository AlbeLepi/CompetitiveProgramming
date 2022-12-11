use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n:usize = n.trim().parse().unwrap();
    let k = 3;
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a:Vec<i64> = a.split(" ").map(|x| x.trim().parse()).collect::<Result<Vec<i64>,_>>().unwrap();
    let mut sum = 0;
    for i in 0..n {sum += a[i];}
    if sum % k as i64 != 0 {println!("0");}
    else if sum == 0 {
        let mut part = 0;
        let mut count = 0;
        let mut res = 0;
        for i in 0..n-1 {
            part += a[i];
            if part == 0 {
                count += 1;
                if count == k-1 {res = 1;}
                if count > k-1 {res = res*count/(count-k+1);}
            }
        }
        println!("{}", res);
    }
    else {
        let q = sum / k;
        let mut part = 0;
        let mut p:Vec<usize> = vec![0; k as usize]; p[0] = 1;
        for i in 0..n {
            part += a[i];
            if part % q == 0 && part / q > 0 && part /q < k {
                p[(part/q) as usize] += p[(part/q - 1) as usize];
            }
        }
        println!("{}", p[(k-1) as usize]);
    }
}
