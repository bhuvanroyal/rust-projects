fn main() {
// by default variables are immutable
    let a=18;
// we can make them mutable by using "mut"
    let mut b =12;
    b=23;
// we declare constants by "const" we must specify type and variable name should be in uppercase
    const VALUE:u32=24;
// this is called shadowing 
    let x=10;
    let x=x+5;
    println!("value of x is {x}");
// Scalar datatypes

    let age: u32= 24;
    let price: f32=234.133;
    let character: char='s';
    let flag: bool=true;

// compound datatypes

    // tuple
    let tup: (bool, f64, i32)=(true,3.14,24);
    println!("{0} and {1}",tup.0,tup.1);
    let (tx,ty,tz)=tup;

    println!("{tx},{ty},{tz}");

    // Array
    let arr=[1,2,3,4,5,6];

    println!("{:?}",arr);

    

    println!("a is ={a}, and b is={b}, and constant value is {VALUE}");
}
