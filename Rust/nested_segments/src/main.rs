use std::io;

fn iniz(ind: usize, interval: &mut Vec<(usize,usize)>, n: usize, poszero: usize) {
    if 2*ind+2 >= interval.len() {
        let pos = if ind >= poszero {ind-poszero} else {ind-poszero+n};
        interval[ind] = (pos,pos);
    } else {
        iniz(2*ind+1, interval, n, poszero);
        iniz(2*ind+2, interval, n, poszero);
        interval[ind] = (interval[2*ind+1].0, interval[2*ind+2].1);
    }
}

fn insert(ind: usize, pos: usize, st: &mut Vec<usize>, interval: &Vec<(usize,usize)>) -> usize {
    if ind >= st.len() || interval[ind].0 > pos {0}
    else if interval[ind].1 < pos {st[ind]}
    else {
        st[ind] += 1;
        insert(2*ind+1, pos, st, interval) + insert(2*ind+2, pos, st, interval)
    }
}

fn main() {
    let mut n = String::new();  io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let mut arr: Vec<(i32,i32,usize)> = Vec::with_capacity(n);
    let mut arrend: Vec<(i32,usize)> = Vec::with_capacity(n);
    for i in 0..n {
        let mut line = String::new();   io::stdin().read_line(&mut line).unwrap();
        let line: Vec<i32> = line.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i32>,_>>().unwrap();
        arrend.push((line[1],i));
        arr.push((line[0],line[1],i));
    }
    arrend.sort();
    for i in 0..n {arr[arrend[i].1].1 = i as i32;}
    arr.sort();
    let mut st: Vec<usize> = vec![0; 2*n-1];
    let mut poszero = 0;
    while 2*poszero+1 < 2*n-1 {poszero = 2*poszero+1;}
    let mut interval:Vec<(usize,usize)> = vec![(0,0); 2*n-1];
    iniz(0, &mut interval, n, poszero);
    let mut risp = vec![0; n];
    for elem in arr.iter().rev() {risp[elem.2] = insert(0, elem.1 as usize, &mut st, &interval);}
    for elem in risp {println!("{} ", elem);}
}
