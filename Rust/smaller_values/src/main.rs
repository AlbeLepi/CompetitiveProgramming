use std::fs;

#[derive(Clone, Copy)]
struct Node {
    value: usize,
    interval: (usize,usize),
    left: usize,
    right: usize,
}

fn iniz(ind: usize, st: &mut Vec<Node>, n: usize, poszero: usize) {
    if 2*ind+2 >= st.len() {
        let pos = if ind >= poszero {ind-poszero} else {ind+n-poszero};
        st[ind].interval = (pos,pos);
    } else {
        st[ind].left = 2*ind+1;
        st[ind].right = 2*ind+2;
        iniz(st[ind].left, st, n, poszero);
        iniz(st[ind].right, st, n, poszero);
        st[ind].interval = (st[st[ind].left].interval.0, st[st[ind].right].interval.1);
    }
}

fn insert(root: usize, st: &mut Vec<Node>, pos: usize) {
    let mut node = st[root];
    node.value += 1;
    if node.left == 0 {st.push(node);}
    else if st[st[root].left].interval.1 < pos {
        node.right = st.len()+1;
        st.push(node);
        insert(st[root].right, st, pos);
    } else {
        node.left = st.len()+1;
        st.push(node);
        insert(st[root].left, st, pos);
    }
}

fn search(st: &Vec<Node>, pos:usize, nod: usize) -> usize {
    if st[nod].left == 0 {st[nod].value}
    else if st[st[nod].left].interval.1 < pos {st[st[nod].left].value + search(st, pos, st[nod].right)}
    else {search(st, pos, st[nod].left)}
}

fn main() {
    for t in 0..9 {
        let text = fs::read_to_string("input/input".to_owned() + &t.to_string() + ".txt").unwrap();
        let lines: Vec<String> = text.lines().map(|x| x.parse()).collect::<Result<Vec<String>, _>>().unwrap();
        let line: Vec<usize> = lines[0].split_whitespace().map(|x| x.parse()).collect::<Result<Vec<usize>, _>>().unwrap();
        let n = line[0];    let m = line[1];
        let line: Vec<usize> = lines[1].split_whitespace().map(|x| x.parse()).collect::<Result<Vec<usize>, _>>().unwrap();
        let mut arr: Vec<(usize,usize)> = Vec::with_capacity(n);
        //coppie (valore, posizione originale)
        for i in 0..n {arr.push((line[i],i));}
        arr.sort();
        let mut positions = vec![0; n];
        //positions[i] = posizione del primo numero > i
        for i in 1..n {
            positions[i] = positions[i-1];
            while positions[i] < n && arr[positions[i]].0 <= i {positions[i] += 1;}
        }
        let node0 = Node {value: 0, interval: (0,0), left: 0, right: 0,};
        let mut st: Vec<Node> = vec![node0; 2*n-1];
        let mut poszero = 0;    while 2*poszero+1 < 2*n-1 {poszero = 2*poszero+1;}
        iniz(0, &mut st, n, poszero);
        let mut roots = Vec::with_capacity(n);  roots.push(0);
        //roots[i+1] = la radice dell'albero dopo in cui ho appena aggiunto arr[i]
        for i in 0..n {
            roots.push(st.len());
            insert(if i == 0 {0} else {roots[i]}, &mut st, arr[i].1);
        }
        let mut res: Vec<usize> = Vec::with_capacity(m);
        for i in 0..m {
            let line: Vec<usize> = lines[i+2].split_whitespace().map(|x| x.parse()).collect::<Result<Vec<usize>, _>>().unwrap();
            let to_i = if line[0] == 0 {0} else {search(&st, line[0]-1, roots[positions[line[2]]])};
            let to_j = search(&st, line[1], roots[positions[line[2]]]);
            res.push(to_j - to_i);
        }
        let text = fs::read_to_string("output/output".to_owned() + &t.to_string() + ".txt").unwrap();
        let lines: Vec<usize> = text.split_whitespace().map(|x| x.parse()).collect::<Result<Vec<usize>, _>>().unwrap();
        if res.len() != lines.len() {println!("Error in input {}", t);} 
        else {
            for j in 0..res.len() {
                if res[j] != lines[j] {println!("Error in input {}", t);}
            }
        }
    }
}
