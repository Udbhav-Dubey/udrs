fn main (){
    another_function(5);
    print_labeled_measurment(4,'h');
    let y={
        let x=3;
        x+1
    };
    println!("the value of y is :{y}"); 
    let x=five();
    println!("the valaue of x is {x}");
    let y=plus_one(5);
    println!("the value of y is {y}");
}
fn plus_one(x:i32)->i32{
    x+1
}
fn five()->i32{
    5
}
fn print_labeled_measurment(value:i32,unit_label:char){
    println!("the measurment is {value}{unit_label}");
}
fn another_function(x:i32){
    println!("the value of x is {x}");
}
