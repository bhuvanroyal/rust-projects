
struct Employee{
    name : String,
    age: u32,
    working_days: u32,
    salary_per_day: u32,
}
/* To define the function within the context of Employee, we start an impl (implementation) block for Employee. 
   Everything within this impl block will be associated with the Employee type. 
   Then we move the calculate_salary function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body. */

/*  ->Each struct is allowed to have multiple impl blocks.
    ->Methods always have self, &self, or &mut self as their first parameter — this refers to the instance the method is being called on.
    */
impl Employee{
    fn calculate_salary(&self)->u32{
        self.working_days*self.salary_per_day
    }

    fn compare_salary(&self, other: &Employee) ->bool{
        self.calculate_salary()>other.calculate_salary()
    }
}

pub fn methods(){

    let emp1=Employee{
        name:String::from("bhuvan"),
        age:24,
        working_days:14,
        salary_per_day:800,
    };

    let emp2=Employee{
        name:String::from("kiran"),
        working_days:20,
        ..emp1
    };

    println!("total salary for working {} days is {}",emp1.working_days,emp1.calculate_salary());
    println!("Is emp1 salary greater than emp2 :{}",emp1.compare_salary(&emp2));

}