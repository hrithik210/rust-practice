use std::vec;

enum BookStatus {
    Availabe,
    CheckedOut(String)
}

struct Book {
    title : String,
    status : BookStatus,
    person : String
}

impl Book {
    fn checkout(&self){
        match &self.status {
            BookStatus::Availabe => println!("{} is avaiable", self.title),
            BookStatus::CheckedOut(person ) => println!("{} is already bought by {}", &self.title , &self.person )
        }
    }
}


fn main(){
    let mut books = vec![Book{
        title : "idk".to_string(),
        status: BookStatus::Availabe,
        person : "".to_string()
    }];
    books[0].checkout();
}