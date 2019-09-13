mod base_types;
//use base_types::Value;


fn syracuse(n:u64)->u64{
    match n%2{
        0 => n/2,
        1 => 3*n+1,
        _ => 0
    }
}

struct SyracuseInfo{
    flight:u64,
    height:u64,
    above_flight:u64
}

fn launch_syracuse(n:u64) -> SyracuseInfo{
    let mut tmp = n;
    let mut fl = 0;
    while tmp!= 1
    {
        fl = fl+1;
        tmp = syracuse(tmp);
    }
    SyracuseInfo {flight :0,height:0, above_flight:0}
}

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
    
    
}

