

fn main(){
    println!("Euler project 1: Multiples of 3 or 5");
    const RANGE:i128 = 1000;

    let sum = compute(RANGE);

    println!("Sum is: {sum}");

}


fn compute(x:i128)->i128{
    let mut sum:i128= 0;

    for x in 1..x{        
        if x % 3 == 0 || x % 5 == 0{
            sum += x;
        }
    }
    sum

}