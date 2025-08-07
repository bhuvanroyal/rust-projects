
// Rust has two main types of Errors
/* 1>Recoverable Errors
        -->Things that can fail, but it's not a crash.
        -->Rust uses the Result<T, E> enum for recoverable errors:
        -->Example: File not found, network request failed, etc.
   2>Unrecoverable Errors
        -->Programming mistakes → things that should never happen.
        -->Rust will panic — i.e., stop the program immediately.
        -->Example: out-of-bounds access, unwrap on None, etc. */

// use core::panic;
use std::{fs::File, io::Read};
use std::io::{self};

fn main() {

    // Rust uses panic!() macro for unrecoverable errors. It crashes the program.
    // panic!("some thing went wrong");


    let file=File::open("C:/Users/damab/rust-projects/rust-projects/chapter_9_Error_handling/src/hello.txt");

    match file{
        Ok(file) =>println!("file is present : {file:?}"),
        Err(error)=>eprintln!("file is not present")
    }

    // unrecoverable errors
    let mut v: Vec<u32>=vec![1,2,3,4,5];
    // println!("{:?}",v[8]);

    // match read_file("demo.txt"){
    //     Ok(contents) => println!("contents are : {}",contents),
    //     Err(err)=>println!("error reading file {}",err)
    // }


    let value: Result<i32, &str> = Ok(42);
    println!("Value: {}", value.unwrap());

    let error: Result<i32, &str> = Err("Something went wrong");
    println!("Error: {}", error.expect("Custom panic message"));

}

fn read_file(file: &str)->Result<String,io::Error>{

    let mut f=std::fs::File::open(file)?;
    let mut contents=String::new();

    f.read_to_string(&mut contents);
    Ok(contents)
}
