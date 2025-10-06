fn main (){
    let x=2.0; // f64
    let y:f32=3.2; // f32
    println!("the value of x is {x} and y is {y}");
    let sum = 5+10;
    let diffrence =95.5-4.4;
    let product = 4*30;
    let quotient = 56.7 / 32.2;
    let trunctated=-5/3; // results in -1
    let remainder = 43%5;
    println!("the sum is {sum}\n,the diffrence is {diffrence}\n,the product is {product}\n,the quotient is {quotient}\n,the trunctated is {trunctated}\n,the remainder is {remainder}\n");
    let t=true;
    let f:bool=false;
    
    let c='z';
    let z:char='ℤ';
    let ghost_emoji ='👻';
    println!("char c is {c} and z is {z} and the emoji is {ghost_emoji}");
    
    let tup:(i32,f64,u8)=(500,6.4,1);
    let tupy=(1000,0.5,55);
    let (a,b,c)=tupy;
    println!("tup is {:?}\ntupy is {:?}\nthe value of b is {}",tup,tupy,b);

    let x:(i32,f64,u8)=(500,6.4,1);
    let five_hundred=x.0;
    let six_point_four=x.1;
    let one =x.2;
    
    let f=[1,2,3,4,5];
    let g:[i32;5]=[1,2,3,4,5];
    let h=[3;5];
    let first =f[0];
    println!("the array is {:?}\nthe first is {:?} and second is {:?}",f,first,f[1]);
    println!("the array g is {:?} \nthe array h is {:?}",g,h);
}
