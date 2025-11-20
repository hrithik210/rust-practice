fn main(){
    let sum = find_sum(5);
    println!("{}", sum)
}

fn find_sum(num : i32) -> i32 {
    let mut total = 0;

    for i  in 1..=num {
        total+=i;
    };

    return total ;
}