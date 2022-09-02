

fn main(){
    println!("Euler project 2: Even fibonacc");
    const RANGE:i128 = 4000000;

    let sum = fib_compute(RANGE);

    println!("Sum is: {sum}");

}


fn fib_compute(limit:i128)->i128{

    let mut n: i128 = 1;
    let mut _n: i128 = 1;


    let mut mut_even_sum:i128 = 0;

    while n < limit {
        let aux = n;
        n = n + _n;
        _n = aux;

        if _n % 2 == 0 {
            mut_even_sum += _n;
        }
    }
    

    mut_even_sum

}