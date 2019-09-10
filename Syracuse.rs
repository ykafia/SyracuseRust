mod base_types;
use base_types::Value;

fn main() {

    
    let mut a = Value {x:0,y:0};
    println!("{}",a);
    while true
    {
        a+=1;
        println!("{}",a);
    }
    
}