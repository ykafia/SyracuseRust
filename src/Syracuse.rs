mod base_types;
use base_types::Value;

fn main() {

    let a:u8 = std::u8::MAX;
    let b = a.checked_sub(1);
    match b {
        Some(x) => {
            println!("{}",x);
            println!("It's a value")
        },
        None =>  println!("Result failed"),
    }
    

    // let mut a = Value {x:9223372036854775806,y:0};
    // println!("{}",a);
    // while a.y<2
    // {
    //     a+=1;
    //     println!("{}",a);
    // }
    
    
}

