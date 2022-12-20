use std::io;
use std::collections::BTreeSet;

fn main() {
    let mut n = String::new();  io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let mut damage = 0; let mut doubles = 0; let mut doubleones = 0; let mut tresh = (0,false);
    let mut spells: BTreeSet<(i64,bool)> = BTreeSet::new();
    for _ in 0..n {
        let mut line = String::new(); io::stdin().read_line(&mut line).unwrap();
        let mut line:Vec<i64> = line.split(' ').map(|x| x.trim().parse()).collect::<Result<Vec<i64>,_>>().unwrap();
        if line[1] > 0 {
            damage += line[1];  spells.insert((line[1],line[0]==1));
            if line[0] == 1 {
                doubles += 1;
                if doubles == 1 {//initialize the treshold
                    tresh = *spells.range(..).next_back().unwrap();
                    if tresh.1 {doubleones += 1;}
                    damage += tresh.0;
                } else if line[1] > tresh.0 {//the new spell gets doubled
                    doubleones += 1;
                    damage += line[1];
                } else {//another spell gets doubled
                    tresh = *spells.range(..tresh).next_back().unwrap();
                    if tresh.1 {doubleones += 1;}
                    damage += tresh.0;
                }
            } else if line[0] == 0 && doubles > 0 && line[1] > tresh.0 {
                damage -= tresh.0;//forget the weakest doubled spell
                if tresh.1 {doubleones -= 1;}
                tresh = *spells.range((tresh.0+1,false)..).next().unwrap();
                damage += line[1];//doubles this spell
            }
        } else if line[1] < 0 {
            line[1] = -line[1];
            damage -= line[1];  spells.remove(&(line[1], line[0]==1));
            if line[0] == 1 {
                doubles -= 1;
                if doubles == 0 {//forget the only doubled spell
                    if tresh.1 {doubleones -= 1;}
                    damage -= tresh.0;
                } else if line[1] > tresh.0 {//the forgotten spell was doubled
                    doubleones -= 1;
                    damage -= line[1];
                } else {//another doubled spell gets forgotten
                    if tresh.1 {doubleones -= 1;}
                    damage -= tresh.0;
                    tresh = *spells.range((tresh.0+1,false)..).next().unwrap();
                }
            }
            else if line[0] == 0 && doubles > 0 && line[1] >= tresh.0 {
                damage -= line[1];
                tresh = *spells.range(..tresh).next_back().unwrap();
                if tresh.1 {doubleones += 1;}
                damage += tresh.0;
            }
        }
        if doubleones == 0 || doubleones < doubles {println!("{}", damage);}
        else {
            let newdoubl = spells.range(..tresh).next_back().unwrap_or(&(0,false));
            println!("{}", damage - tresh.0 + newdoubl.0);
        }
    }
}