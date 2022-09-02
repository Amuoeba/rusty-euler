
use std::thread;
use std::time::Duration;

fn main(){





    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(500));
    }




    println!("Euler project 3: Sum square difference");
    const RANGE:i128 = 4000000;

    let sum = compute(RANGE);

    println!("Sum is: {sum}");


    let mut s =  String::from("erikjan");
    let name =  &s[0..4];
    println!("String:{s}");
    println!("Name: {name}");

    s.replace_range(0..1, "x");

    println!("String:{s}");
    println!("Name: {name}");
    

}


fn compute(limit:i128)->i128{

    return 0;
}