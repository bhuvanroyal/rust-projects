
pub fn string_examples(){
    let s1="hello";
    let s2="world";
    let mut s3=String::from("this is new String");
    let mut s4=String::from("another string");
    s3.push_str(s2);
    println!("{s3}");
    let s5=s3+&s4;
    let s6=s4.push_str(" pushed string");

    println!("{s2}");
    println!("{s4}");

    // we can concatenate the strings by using format!() marcro it takes only references

    let t1="this";
    let t2=String::from("is");
    let t3=String::from("apple");

    let t=format!("{t1} {t2} {t3}");
    println!("{t}");

    println!("t1 is {t1}, and t2 is {t2}");

    // String slicing
    let sl1=&t[0..6];
    println!("{sl1}");

}