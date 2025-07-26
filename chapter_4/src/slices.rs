
pub fn slices(){
    let s = String::from("hello world");

    let h = &s[0..5];
    let w = &s[6..11];

    println!("h is {h}, w is {w}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}