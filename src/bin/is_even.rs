fn main(){
    let n :i32 = 5;

    for i in 1..=n {
        if is_even(i){
            println!("{}", i);
        }
    }
}

fn is_even(num : i32) -> bool {
  
    if num % 2 == 0 {
        return true
    }else {
        return false
    }
}