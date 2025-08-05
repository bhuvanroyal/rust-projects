use std::collections::HashMap;


pub fn maps_example(){

    println!("---------HashMaps---------");

//  for dynamic data — prefer HashMap<String, T>
    let mut hm: HashMap<String, i32> = HashMap::new();
    hm.insert("banana".to_string(), 5);


    let mut hm:HashMap<&str,i32>=HashMap::new();

    hm.insert("bhuvan", 24);
    hm.insert("kiran", 26);

    let age=hm.get("kiran").copied().unwrap_or(0);

    println!("{age}");

    println!("{:#?}",hm);

// Adding a Key and Value Only If a Key Isn’t Present

    hm.entry("shiva").or_insert(22);
    hm.entry("bhuvan").or_insert(28);

    for (k,v) in &hm{
        println!("{k} : {v}");
    }



    

}