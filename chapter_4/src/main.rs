mod ownership;
mod references;
mod slices;
fn main() {
    let x = 5;
    let y = x;
/*  x is still valid and wasn’t moved into y.The reason is that types such as integers that have a known size at compile time
    are stored entirely on the stack, so copies of the actual values are quick to make */
    println!("x = {x}, y = {y}");


    // ownership::string_demo();

    let s1=String::from("bhuvan");
//  here the value of s1 is moved to s2 and ownership also moved, after assigning s11 to s2 if we try to print s1 it wont compile
    let s2=s1;
    println!("{s2}");

// If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    let s3=s2.clone();
    println!("s2 is :{s2}, s3 is :{s3}");

    let temp=String::from("hello from temp");
    take_ownership(temp); //temp value move into function
    // println!("{temp}"); , so no longer valid here


//  Returning values can also transfer ownership
    let st1=gives_ownership();
    println!("{st1}");

    let st2=takes_and_gives_back(st1);
    println!("{st2}");
    // println!("{st1}"); , st1 moved to st2, no longer valid

    references::references();
    slices::slices();

}

fn take_ownership(st: String){
    println!("{st}");
}

fn gives_ownership() ->String{
    let s=String::from("hello from gives_ownership");
    s
}

fn takes_and_gives_back(st: String) -> String{
    st
}