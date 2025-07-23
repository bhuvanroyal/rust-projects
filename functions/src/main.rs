
fn add(a: u32,b :u32){
    let total=a+b;
    println!("The sum is {total}");
}

fn find_primes(x :u32,y :u32)->Vec<u32> {
    let mut arr:Vec<u32>=Vec::new();
    let mut x=x;
    while x<=y {
        let mut flag:bool=true;
        for i in 2..x{
            if x%i==0{
                flag=false;
                break;
            }
        }
        if flag{
            arr.push(x);
        }
        x=x+1;
    }
    arr
}

fn main() {
    add(2,5);
    // find the prime numbers between the ranges
    let x=5;
    let y=15;
    let v: Vec<u32>=find_primes(x,y);
    println!("prime numbers between {0}, {1} are",x,y);
    for i in v{
        println!("{}",i);
    }
}
