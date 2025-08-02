mod match_control_flow;


//  enums give you a way of saying a value is one of a possible set of values. 
#[derive(Debug)]
enum IpAddrKind{
    Ipv4,
    Ipv6,
}
// in general we will create a struct and provide address field to store address or any metadata that is related to this AddrKind
struct AddrKind{
    kind : IpAddrKind,
    address : String,
}
/* However, representing the same concept using just an enum is more concise: rather than an enum inside a struct, 
    we can put data directly into each enum variant. */
#[derive(Debug)]
enum IpAddrKindData{
    Ipv4(String),
    Ipv6(String),
}

/*There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data. */
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/*just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. */
impl Message{
    fn call(&self){
        println!("{:?}",self);
    }
}



fn main() {
// both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. 
   let four=IpAddrKind::Ipv4;
   let six=IpAddrKind::Ipv6;

   let home=AddrKind{
    kind : IpAddrKind::Ipv4,
    address: String::from("127.0.0.1"),
   };

   let four=IpAddrKindData::Ipv4(String::from("121.2.0.125"));
   let six=IpAddrKindData::Ipv6(String::from("125.0.0.1.150"));

   let m=Message::Write(String::from("bhuvan"));
   m.call();
   println!("{:#?}", four);

// OPTION ENUM
/*Option, which is another enum defined by the standard library. The Option type encodes the very common scenario in which 
    a value could be something or it could be nothing. */
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    
    match_control_flow::match_control();

}
