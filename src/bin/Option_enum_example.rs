
fn main(){
    let str = String::from("hallow");
    let ans = find_first_a(&str);

    match ans {
        None => println!("cant find the index"),
        Some(val) => println!("first index of a is {val}")
    }

}

fn find_first_a(s : &String) -> Option<u32> {
    
    let mut index = 0;

    for c in s.chars(){

        index = index +1 ;
        if c == 'a' {
            return Some(index);
        }
    
    }
    None
}