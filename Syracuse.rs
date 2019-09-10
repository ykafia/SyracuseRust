mod base_types;
use base_types::Value;

fn clear() {
    if std::process::Command::new("cls").status().unwrap().success() {
        println!("screen successfully cleared");
    }
}


fn main() {

    
    let  a = Value {x:0,y:0};
    println!("{}",a);
    clear();
    // while a.y<1
    // {
    //     a+=1;
    //     println!("{}",a);

    // }
    
}

