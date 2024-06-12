// struct User{
//     username : String,
//     email : String,
//     sign_in_count: u64,
//     active: bool,
// }


// fn main(){
//     let user1 = User{
//         username:String::from("saurabh"),
//         email : String::from("saurabhyadav@gmail.com"),
//         sign_in_count: 1,
//         active: true
//     };
//    

//     let user2 = buil_user_with_function(String::from("sdwedef"),String::from("wdwed"));

// }

// fn buil_user_with_function(email: String,username: String) -> User {
//     User{
//         username,
//         email,
//         sign_in_count: 1,
//         active: true,
//     }
// }


#[derive(Debug)]
struct Rect{
    height:u64,
    width: u64
}


impl Rect{
    
    //Methods
    fn area(&self) -> u64 {
        self.width * self.height
    }
    fn can_hold(&self,other:&Rect) -> bool {
        self.width >other.width && self.height > other.height
    }
}

impl Rect{
    //Associated function
    fn square(size:u64) -> Rect{
        Rect { 
            height: size, width: size
        }
        
    }
}


fn main(){
    let rect1 = Rect{ height:12,width:43};
    let area1 = rect1.area();
    println!("rectangle 1 is {:#?}",rect1);
    println!("area of the rectnagle is {area1}");

    let rect2 = Rect{
        height: 24,
        width: 45,
    };
    println!("rectangle 1 is {:#?}",rect2);
    println!("area of the rectnagle is {}",rect2.area());

    println!("Can rect2 can hold rect1 {}",rect2.can_hold(&rect1));

    println!("rectangle 3 is {:#?}",Rect::square(2));



}