mod string_demo;
mod maps_demo;

fn main() {

// Vectors
/*A Vector (Vec<T>) is a growable, heap-allocated list that can store multiple values of the same type. */

// Empty vector
let mut v: Vec<i32>=Vec::new();
v.push(1);
v.push(2);
v.push(3);

// Using the macro (vec![])
let mut v1=vec![10,20,30,40];
v1.push(50);
v1.pop();
v1.push(60);
println!("{:?}",v1);
v1.remove(3);
println!("{:?}",v1);

// Reading vector values

let e1=v.get(2);
match e1{
    Some(x) => println!("{x}"),
    None =>println!("no value")
}
let e2=&v1[3];
println!("{:?}",e2);

if let Some(x)=v.get(2){
    println!("value at index 2 is{x}");
}

// iterating over the vector

for i in &mut v{
    println!("{i}");
    *i=*i+100;
}

println!("after changing {:?}",v);
string_demo::string_examples();
maps_demo::maps_example();

}
