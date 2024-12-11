use std::io::stdin;

fn main() {
    let mut s=String::new();
    let mut sum=0;

    loop {
        stdin().read_line(&mut s).unwrap();
        if s.trim().is_empty(){
            break;
        }
       let dd:Vec<&str>= s.split_whitespace().collect();
        let mut is_safe =true;
        let mut is_incr=-2;
       let ss:Vec<i32>= dd.iter().map(|x| x.parse::<i32>().unwrap()).collect();
            let mut prev=-1;
            for asd in ss{

                if prev!=-1{
                  let sss=asd-prev;
                    if sss.abs()>3||sss.abs()<1{
                        is_safe =false;
                        break;
                    }

                    if is_incr==-2{
                        if sss==0{
                            is_safe =false;
                            break;
                        }
                        is_incr=sss/(sss.abs());
                    }
                    else{
                        let check_is_incr=sss/(sss.abs());
                        if check_is_incr!=is_incr{
                            is_safe =false;
                            break;
                        }


                    }

                }

                prev=asd;

            }
        if is_safe{
            sum=sum+1;
        }


        s="".to_string();
    }

    println!("{}",sum);
}