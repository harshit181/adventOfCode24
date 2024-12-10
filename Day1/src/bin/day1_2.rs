use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Read, Write};

fn main() {

    let mut s=String::new();
    let mut left:HashMap<i32,i32>=HashMap::new();
    let mut  right:HashMap<i32,i32>=HashMap::new();
    loop{
        stdin().read_line(&mut s).unwrap();
        if s.trim().is_empty(){
            break;
        }
        let sx=s.trim();
        let xx:Vec<&str>=sx.split_whitespace().collect();
        let l:i32=xx[0].parse().unwrap();
        if !left.contains_key(&l){
            left.insert(l,0);
        }
        let ss=left.get(&l).unwrap()+1;
        left.insert(l,ss);

        let r:i32=xx[1].parse().unwrap();

        if !right.contains_key(&r){
            right.insert(r,0);
        }
        let ss=right.get(&r).unwrap()+1;
        right.insert(r,ss);
        s="".to_string();
    }
    let mut sum=0;


    for (a,(b,c)) in left.iter().enumerate(){

        if right.contains_key(b){

            let d=right.get(b).unwrap();
            sum=sum+(b*c*d);
        }

    }


    println!("{}",sum);

}
