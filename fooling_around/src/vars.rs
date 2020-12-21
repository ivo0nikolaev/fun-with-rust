// NB! Variables here are IMMUTABLE by default
// Vars hold primitive data or references to data
// Block-scoped

// var_name is the proper formating

pub fn run() { //Public 
    
    //These are immutable
    let character_name = "Sam";
    let class_name = "mage";

    // let mut makes a variable mutable
    let mut gold = 20;

    
    println!("{} is an aspiring {}, but only has {} gold coins!",
     character_name, class_name, gold);

    //Having a muttable variable

    gold = 3;

    println!("Unfortunately {} got drung last night and only has {} gold left",
     character_name, gold);

    
     // Const - we need to specify const NAME: type = value, for it to work

     const ID: i32 = 022;

     println!("Charachter ID is {}", ID);

    //Assign multiple variables 
    let ( new_name, new_class, new_gold) = ("Jack", "rouge", 7);

    // 
    println!("{} is some {}, who has like {} gold coins!",
    new_name, new_class, new_gold);

}