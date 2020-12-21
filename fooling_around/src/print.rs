pub fn  run() {
    //Print stuff

    println!("Printing from print.rs");

    //Can't directly print numbers. Needs string literals.
    //Can use placehodlers with {}
    println!("Name {}", "Joe");

    // Multiple placeholders
    println!("{} are friends {}", "Joe", "Sam");

    // Mulstiple positional args with zero-base indexing. Order is important
    println!("{2} and {0}, went to visit {1}", "Nina", "Becca", "Sarah");

    //Named Args
    println!("{name} loves {food}", name = "Jackie", food = "muffins" );

    // Placeholder traits
    println!("Binary: {:b} Hex; {:x} Octal: {:o}", 27, 27, 27);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

}