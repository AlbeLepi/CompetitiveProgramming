use std::io;

fn iniz(st:&mut [i64], interval:&mut Vec<(usize,usize)>, n:usize, poszero:usize, nums:&[i64]) {
    for ind in (0..interval.len()).rev() {
        if 2*ind+2 >= interval.len() {
            let pos = if ind >= poszero {ind-poszero} else {ind-poszero+n};
            interval[ind] = (pos,pos);
            st[ind] = nums[pos];
        } else {
            interval[ind] = (interval[2*ind+1].0, interval[2*ind+2].1);
            st[ind] = if st[2*ind+1] > st[2*ind+2] {st[2*ind+2]} else {st[2*ind+1]};
        }
    }
}

fn inc(i:usize, j:usize, d:i64, st:&mut [i64], fm:&mut [i64], interval:&[(usize,usize)]) {
    if d == 0 {return;}
    let mut queue:Vec<usize> = vec![0; 1];
    let mut sc = 0;
    while sc < queue.len() {
        if interval[queue[sc]].0 < i || interval[queue[sc]].1 > j { 
            if interval[2*queue[sc]+1].0 <= j && interval[2*queue[sc]+1].1 >=i {queue.push(2*queue[sc]+1);}
            if interval[2*queue[sc]+2].0 <= j && interval[2*queue[sc]+2].1 >=i {queue.push(2*queue[sc]+2);}
        }
        sc += 1;
    }
    for &ind in queue.iter().rev() {
        if interval[ind].0 >= i && interval[ind].1 <= j {st[ind] += d; fm[ind] += d;}
        else {st[ind] = fm[ind] + if st[2*ind+1] > st[2*ind+2] {st[2*ind+2]} else {st[2*ind+1]};}
    }
}

fn rmq(i:usize, j:usize, st:& [i64], fm:&[i64], answer:&mut [i64], interval:&[(usize,usize)]) -> i64 {
    let mut queue:Vec<usize> = vec![0; 1];
    let mut sc = 0;
    while sc < queue.len() {
        if interval[queue[sc]].0 < i || interval[queue[sc]].1 > j {
            if interval[2*queue[sc]+1].0 <= j && interval[2*queue[sc]+1].1 >=i {queue.push(2*queue[sc]+1);}
            if interval[2*queue[sc]+2].0 <= j && interval[2*queue[sc]+2].1 >=i {queue.push(2*queue[sc]+2);}
        }
        sc += 1;
    }
    for &ind in queue.iter().rev() {
        if interval[ind].0 >= i && interval[ind].1 <= j {answer[ind] = st[ind];}
        else if interval[2*ind+1].1 >= j {answer[ind] = fm[ind] + answer[2*ind+1];}
        else if interval[2*ind+2].0 <= i {answer[ind] = fm[ind] + answer[2*ind+2];}
        else {answer[ind] = fm[ind] + if answer[2*ind+1] > answer[2*ind+2] {answer[2*ind+2]} else {answer[2*ind+1]};}
    }
    answer[0]
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n:usize = n.trim().parse().unwrap();
    let mut nums=String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums:Vec<i64>=nums.split_whitespace()
        .map(|x| x.parse()).collect::<Result<Vec<i64>,_>>().unwrap();
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let m:usize = m.trim().parse().unwrap();
    let mut st:Vec<i64> = vec![0; 2*n-1];
    let mut fm:Vec<i64> = vec![0; 2*n-1];
    let mut poszero = 0;
    while 2*poszero+1 < 2*n-1 {poszero = 2*poszero+1;}
    let mut interval:Vec<(usize,usize)> = vec![(0,0); 2*n-1];
    let mut answer = vec![0; 2*n-1];
    iniz(&mut st, &mut interval, n, poszero, &nums);
    for _ in 0..m {
        let mut line=String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line:Vec<i64>=line.split_whitespace()
            .map(|x| x.parse()).collect::<Result<Vec<i64>,_>>().unwrap();
        if line.len() == 2 {
            let v =
            if line[0] <= line[1] {rmq(line[0] as usize, line[1] as usize, &st, &fm, &mut answer, &interval)}
            else {
                let a = rmq(0, line[1] as usize, &st, &fm, &mut answer, &interval);
                let b = rmq(line[0] as usize, n-1, &st, &fm, &mut answer, &interval);
                if a < b {a} else {b}
            };
            println!("{}", v);
        } else {
            if line[0] <= line[1] {
                inc(line[0] as usize, line[1] as usize, line[2], &mut st, &mut fm, &interval);
            } else {
                inc(0, line[1] as usize, line[2], &mut st, &mut fm, &interval);
                inc(line[0] as usize, n-1, line[2], &mut st, &mut fm, &interval);
            }
//            for elem in st.iter() {print!("{} ", elem);} println!();
        }
    }
}
