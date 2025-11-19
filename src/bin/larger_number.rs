
fn main(){
    let num = larger_number(3, 4) ;
    println!("{} is greater number", num);
}



fn larger_number(a : i32 , b : i32) -> i32 {
    if a > b {
        return a;
    }else{
        return b;
    }
}