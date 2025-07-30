mod Rectangle_functions;
mod method_syntax;

struct User{
    active: bool,
    name : String,
    email: String,
    age : u32,
}
#[derive(Debug)]
struct Rectangle{
    width: u32,
    length: u32,
}

fn main() {
    let user1=User{
        active:true,
        age:24,
        name:String::from("bhuvan"),
        email:String::from("bhuvan@gmail.com"),
    };
    
    println!("user1 name {} and age is {}",user1.name,user1.age);
//  we can even modify the struct field data if we declare user1 as mut
//  user1.age=25;

//  we can create a user2 instance with all the data from user1, and we can also provide explicit fields
    let user2=User{
        name:String::from("kiran"),
        ..user1
    };
    println!("user2 name is {}",user2.name);

    let length: u32=4;
    let width: u32=8;
    println!("Area of the rectangle is {}", Rectangle_functions::area(length,width));


    let rect1=Rectangle{
        width:6,
        length:3,
    };
/*  we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, 
    which is the reason we use the & in the function signature and where we call the function. */
    println!("Area of the rectangle is {}",rect_area(&rect1));
    println!("rect1 width is {}",rect1.width);

/*  to print these structs in a debug mode we have to declare "#[derive(Deubg)]" before the struct and we can use
    {rect1:?} or {rect1:#?} or dbg!(&rect1) */
    dbg!(&rect1);
    println!("rect1 is {rect1:?}");


    method_syntax::methods();


}

fn rect_area(rect1: &Rectangle) -> u32{
    rect1.length * rect1.width
}