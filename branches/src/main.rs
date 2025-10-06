use std::io;
fn main (){
    println!("please enter an integer : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number : i32 = input.trim().parse().expect("please enter a valid number");
    if number < 5{
        println!("condition was true");
    }
    else {println!("condition was false");}
    
    if number %4==0{
        println!("the number is divisible by 4");
    } else if number %3==0{
        println!("the number is divisible by 3");
    }else if number %2==0{
        println!("the number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4,3,2");
    }
}
