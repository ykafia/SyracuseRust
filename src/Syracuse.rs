mod base_types;
use base_types::*;

fn syracuse(n: u64) -> u64 {
    match n % 2 {
        0 => n / 2,
        1 => 3 * n + 1,
        _ => 1,
    }
}

fn syracuse1(n: u64, verbose: bool) -> SyracuseInfo {
    let mut info = SyracuseInfo {
        flight: 0,
        height: 0,
        above_flight: 0,
        updated_above: false,
        init: n,
        seen: Vec::new(),
    };
    info.seen.push(n);
    let mut x:u64 = match info.seen.last()
    {
        Some(x) => *x,
        None => 1u64,
    };
    while x != 1u64
    {
        
        if verbose {
            println!("{} -- val : {}\n", info.flight, x);
        }
        info.flight += 1;

        info.seen.push(syracuse(x));
        if x < info.init && !info.updated_above {
            info.above_flight = info.flight;
            info.updated_above = true;
        }
        if info.height < x {
            info.height = x;
        }
        x = match info.seen.last()
        {
            Some(x) => *x,
            None => 1u64,
        };
    }
    if verbose {
        println!("{} -- val : {}\n", info.flight, x);
    }
    info.seen = unique(info.seen);
    return info;
}

fn syracuse2(limit: u64) -> TypeLists {
    let mut count: u64 = 1;
    let mut prev_syr: Vec<SyracuseInfo> = Vec::new();
    let mut types = TypeLists {
        type1: Vec::new(),
        type2: Vec::new(),
        type3: Vec::new(),
        type4: Vec::new(),
    };
    loop {
        if count >= limit {
            println!("Everything is finished, needs the type 1 check");
            types.type1 = list_type1(&prev_syr);
            return types;
        }
        
        let tmp = syracuse1(count, false);
        if is_type2(&mut prev_syr, &tmp) {
            types.type2.push(count);
        }
        if is_type3(&mut prev_syr, &tmp) {
            types.type3.push(count);
        }
        if is_type4(&mut prev_syr, &tmp) {
            types.type4.push(count);
        }
        prev_syr.push(tmp);
        count += 2;
    }
}

fn main() {
    //println!("Flight info of 7 : {}", syracuse1(7, false));

    let search = syracuse2(50);
    types_head(&search);
}
