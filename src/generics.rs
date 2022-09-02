




fn find_largest(list: &[i32]) -> &i32{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    println!("Generics rustbook!");

    let number_list = vec![20,21,22,23,220,44,42,99];
    let number_list_1: Vec<i32> = vec![10,11,9,33,9,8];


    let largest = find_largest(&number_list);
    let largest1 :&i32 = find_largest(&number_list_1);

    println!("The largest number is: {largest}");
    println!("The largest number is: {largest1}");
    

}






