use std::io::stdin;
use regex::Regex;

fn main() {
    let mut s=String::new();
    let mut s2=String::new();
    let mut sum=0;
    loop {
        stdin().read_line(&mut s).unwrap();
        if s.trim().is_empty() {
            break;
        }
        s2=s2+ s.as_str();
        s="".to_string();
    }
    let ss=s2.replace(" ","");
    let ss=ss.replace("\n","");
    let ss=ss.replace("\r","");
        let re=Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let azzs=ss.split("do()");

        for asz in azzs{
            let azqw:Vec<&str>=asz.split("don't()").collect();
            for dds in re.captures_iter(azqw[0]){
                let number1:i32=(&dds[1]).parse().unwrap();
                let number:i32=(&dds[2]).parse().unwrap();
                sum=sum+number1*number;
            }

        }

        // let ss=s.find("mul(").unwrap();





    println!("{}",sum);


}