use std::io::stdin;
use regex::Regex;

fn main() {
    let mut s=String::new();
    let mut sum=0;
    loop{
        stdin().read_line(&mut s).unwrap();
        if s.trim().is_empty(){
            break;
        }
        let ss=s.replace(" ","");
        let re=Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        // let ss=s.find("mul(").unwrap();

        for dds in re.captures_iter(ss.as_str()){
            let number1:i32=(&dds[1]).parse().unwrap();
            let number:i32=(&dds[2]).parse().unwrap();
            sum=sum+number1*number;
        }
        s="".to_string();
    }


    println!("{}",sum);


}