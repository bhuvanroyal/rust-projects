


pub fn references(){
//  Rust has a feature for using a value without transferring ownership, called references.

let s=String::from("hello");
let s1=&s;
println!("s is {s}, s1 is {s1}");

//  So, what happens if we try to modify something we’re borrowing?
// change(&s);

//  Mutable references
// we can modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:

    let mut temp=String::from("hello from temp");
    change(&mut temp);
    println!("{temp}");

/*   we cannot borrow temp as mutable more than once at a time.
    -> we can use curly brackets to create a new scope, allowing for multiple mutable references,
    -> We also cannot have a mutable reference while we have an immutable one to the same value.
    */




}

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(s: &mut String){
    s.push_str(" from change");
}