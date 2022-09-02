fn main(){
    println!("Helo there from new binary");

    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    {   
        println!("########## In inner scope ##########");
        let mut x = x + 1;
        println!("The value of x is {x}");
        x = 8;
        println!("The value of x is {x}");
        println!("####################################")
    }
    println!("The value of x is {x}");

    let b = 0x11;
    println!("b = {b}");


    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let a1: [i8;5] = [1,2,3,4,5,];
    let tup = (1,'z');
    // Specify initiall values and lenght of the array
    let a2 = [tup; 4];
    println!("{}",a1[2]);
    println!("{}",a2[3].1);

    test_function();
    let ret = retunr_value_test(true);
    println!("Return value is {ret}");

    for_loop_example();


    let mut test_string = String::from("Erik jan test");

    test_string.push_str("pushed string");

    println!("Test string is {}",test_string);


    let s = String::from("Peter janežič");

    println!("First word: {}",return_first_word(&s));

    println!("Add one: {:?}",  add_one(Some(1)));
    println!("Add one: {:?}",  add_one(None));

}


fn test_function(){
    println!("This print is from another function");
}

fn retunr_value_test(toggle:bool)->i32{

    // Important to note here. In rust expressions dont have a semicolon at the end. Functions without semicolons implicitly return 
    // the value of last line
    if toggle {
        return 10;
    }
    20
}

fn for_loop_example(){
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}


fn return_first_word(s: &String) -> &str{
    let string_bytes = s.as_bytes();
    for (i,&item) in string_bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


fn add_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}