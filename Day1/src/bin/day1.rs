use std::io::{stdin, stdout, Read, Write};

fn main() {

    let mut s=String::new();
    let mut left:Vec<i32>=Vec::new();
    let mut  right:Vec<i32> =Vec::new();
    loop{
        stdin().read_line(&mut s).unwrap();
        if s.trim().is_empty(){
            break;
        }
        let sx=s.trim();
        let xx:Vec<&str>=sx.split_whitespace().collect();
        let l:i32=xx[0].parse().unwrap();
        left.push(l);
        let r:i32=xx[1].parse().unwrap();
        right.push(r);
        s="".to_string();
    }

    left.sort();
    right.sort();
    let mut sum=0;
    for (a,b) in left.iter().enumerate(){
        let rg=right[a]-b;
        sum=sum+rg.abs();

    }
    println!("{}",sum);

}
