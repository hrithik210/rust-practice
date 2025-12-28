fn main(){
    let mut s1 = String::from("Hello");
    s1.push_str(" hey");

    let s2 = &mut s1;
    s2.push_str(" hi");

    let s3 = &s1;
    let s4  = &s1;

    println!("{s1}")
}