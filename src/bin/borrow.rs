fn main(){
    let a = String::from("hrithik");
    let len = get_length(&a);

    println!("lenght of {} is {}",a,  len)
}

fn get_length(s : &String) -> usize{
    return s.len()
}