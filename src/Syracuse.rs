mod base_types;
use base_types::SyracuseInfo;


fn syracuse(n:u64)->u64{
    match n%2{
        0 => n/2,
        1 => 3*n+1,
        _ => 1
    }
}



fn syracuse1(n:u64, verbose:bool) -> SyracuseInfo{
    let mut info = SyracuseInfo {flight:0,height:0,above_flight:0,updated_above:false,current:n,init:n};
    while info.current!= 1
    {
        info.flight+=1;
        if verbose{
            println!("{} -- val : {}\n",info.flight,info.current);
        }
           
        info.current = syracuse(info.current);
        if info.current < info.init && !info.updated_above
        {
            info.above_flight = info.flight;
            info.updated_above = true;
        }
        if info.height<info.current
        {
            info.height = info.current;
        }
    }
    if verbose{
        println!("{} -- val : {}\n",info.flight,info.current);
    }
    return info;
}
fn Ex1(){
     println!("{}",syracuse1(7,true));
}
fn main() {

    Ex1();

}


