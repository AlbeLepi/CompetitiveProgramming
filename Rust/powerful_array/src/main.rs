use std::io;

fn main() {
    let mut line = String::new();   io::stdin().read_line(&mut line).unwrap();
    let line: Vec<usize> = line.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<usize>,_>>().unwrap();
    let n = line[0];    let t = line[1];
    let mut line = String::new();   io::stdin().read_line(&mut line).unwrap();
    let nums: Vec<usize> = line.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<usize>,_>>().unwrap();
    let mut queries: Vec<(usize,usize,usize)> = Vec::with_capacity(t);
    for i in 0..t {
        let mut line = String::new();   io::stdin().read_line(&mut line).unwrap();
        let line: Vec<usize> = line.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<usize>,_>>().unwrap();
        queries.push((line[0]-1,line[1],i));
    }
    queries.sort();
    let mut sqrtn = 0;   while sqrtn*sqrtn < n {sqrtn += 1;}
    let mut risp: Vec<i64> = vec![0; t];
    let mut start = 0;  let mut end = 0;    let mut count: i64 = 0;
    let mut dict: Vec<i64> = vec![0; 1000001];
    while !queries.is_empty() {
        let mut queue: Vec<(usize,usize,usize)> = Vec::with_capacity(sqrtn);
        for _ in 0..sqrtn {if !queries.is_empty() {
            let temp = queries.pop().unwrap();
            queue.push((temp.1,temp.0,temp.2));
        }}
        queue.sort();
        for elem in queue {
            while start > elem.1 {
                start -= 1;
                count += (nums[start] as i64)*(2*dict[nums[start]]+1);
                dict[nums[start]] += 1;
            }
            while start < elem.1 {
                count += (nums[start] as i64)*(-2*dict[nums[start]]+1);
                dict[nums[start]] -= 1;
                start += 1;
            }
            while end < elem.0 {
                count += (nums[end] as i64)*(2*dict[nums[end]]+1);
                dict[nums[end]] += 1;
                end += 1;
            }
            while end > elem.0 {
                end -= 1;
                count += (nums[end] as i64)*(-2*dict[nums[end]]+1);
                dict[nums[end]] -= 1;
            }
            risp[elem.2] = count;
        }
    }
    for elem in risp {println!("{} ", elem);}
}
