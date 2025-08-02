
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin : Coin) -> u8{
     match coin{
        Coin::Nickel =>5,
        Coin::Penny =>{
            println!("This is penny coin");
            1
        }
        Coin::Dime =>10,
        Coin::Quarter =>25,
    }
}

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

   



pub fn match_control(){

    /*Rust has an extremely powerful control flow construct called match that allows you to compare 
    a value against a series of patterns and then execute code based on which pattern matches */

    value_in_cents(Coin::Penny);
   
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice: u8=5;

    //  We no longer need to use the catch-all value, so we can change our code to use _ instead of the variable named other:
    match dice {
        1 => {println!("one step up");}
        2 => {println!("two steps right")}
        3 => {println!("re roll the dice")}
        other => println!("other")
    }



}