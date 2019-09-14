use std::fmt;
use std::ops::{Add, AddAssign, Sub};

pub struct Value {
    pub x: u64,
    pub y: u64,
}

impl fmt::Display for Value {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct SyracuseInfo {
    pub flight: u64,
    pub height: u64,
    pub above_flight: u64,
    pub updated_above: bool,
    pub init: u64,
    pub seen: Vec<u64>,
}

#[derive(Clone)]
pub struct TypeLists {
    pub type1: Vec<u64>,
    pub type2: Vec<u64>,
    pub type3: Vec<u64>,
    pub type4: Vec<u64>,
}

impl fmt::Display for SyracuseInfo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Flight : {}\nHeight : {}\nAbove Flight : {}\n",
            self.flight, self.height, self.above_flight
        )
    }
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        let added = self.x.checked_add(other.x);
        match added {
            Some(_x) => Value {
                x: self.x + other.x,
                y: self.y + other.y,
            },
            None => {
                let rest = std::u64::MAX - self.x;
                let unit = other.x - rest;
                Value {
                    x: unit,
                    y: self.y + other.y + 1,
                }
            }
        }
    }
}

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        let added = self.x.checked_sub(other.x);
        match added {
            Some(_x) => Value {
                x: self.x - other.x,
                y: self.y - other.y,
            },
            None => {
                let rest = other.x - self.x;
                let unit = std::u64::MAX - rest;
                Value {
                    x: unit,
                    y: self.y - other.y - 1,
                }
            }
        }
    }
}

impl Add<u64> for Value {
    type Output = Value;
    fn add(self, other: u64) -> Value {
        let added = self.x.checked_add(other);
        match added {
            Some(_x) => Value {
                x: self.x + other,
                y: self.y,
            },
            None => {
                let rest = std::u64::MAX - self.x;
                let unit = other - rest;
                Value {
                    x: unit,
                    y: self.y + 1,
                }
            }
        }
    }
}

impl Sub<u64> for Value {
    type Output = Value;
    fn sub(self, other: u64) -> Value {
        let added = self.x.checked_add(other);
        match added {
            Some(_x) => Value {
                x: self.x - other,
                y: self.y,
            },
            None => {
                let rest = other - self.x;
                let unit = std::u64::MAX - rest;
                Value {
                    x: unit,
                    y: self.y - 1,
                }
            }
        }
    }
}
impl AddAssign<u64> for Value {
    fn add_assign(&mut self, other: u64) {
        let added = self.x + other;
        if added < self.x {
            self.x = self.x + other;
            self.y = self.y + 1;
        } else {
            self.x = self.x + other;
        }
    }
}

// Check syracuse types here

fn max_flight(arr: &Vec<SyracuseInfo>) -> u64 {
    let mut max = 0u64;
    for i in arr.iter() {
        if i.flight > max {
            max = i.flight;
        }
    }
    return max;
}

fn max_height(arr: &Vec<SyracuseInfo>) -> u64 {
    let mut max = 0u64;
    for i in arr.iter() {
        if i.height > max {
            max = i.height;
        }
    }
    return max;
}
fn max_above(arr: &Vec<SyracuseInfo>) -> u64 {
    let mut max = 0u64;
    for i in arr.iter() {
        if i.above_flight > max {
            max = i.above_flight;
        }
    }
    return max;
}

pub fn list_type1(prev: &Vec<SyracuseInfo>) -> Vec<u64>{
    let mut uniques:Vec<u64> = Vec::new();
    let mut result:Vec<u64> = Vec::new();

    for i in prev.iter()
    {
        uniques.extend(&i.seen);
    }
    uniques = unique(uniques);
    let maxval = match uniques.last()
    {
        Some(x) => x,
        None => &0u64
    };
    for x in 1u64..*maxval
    {
        if uniques.contains(&x) && &x%2!=0
        {
            result.push(x);
        }
        
    }
    return result;
    
    
}

pub fn is_type2(prev: &Vec<SyracuseInfo>, syr: &SyracuseInfo) -> bool {
    let max = max_flight(prev);
    if max < syr.flight {
        return true;
    } else {
        return false;
    }
}

pub fn is_type3(prev: &Vec<SyracuseInfo>, syr: &SyracuseInfo) -> bool {
    let max = max_height(prev);
    if max < syr.height {
        return true;
    } else {
        return false;
    }
}

pub fn is_type4(prev: &Vec<SyracuseInfo>, syr: &SyracuseInfo) -> bool {
    let max = max_above(prev);
    if max < syr.above_flight {
        return true;
    } else {
        return false;
    }
}

pub fn types_head(data: &TypeLists) {
    println!("List of Type 1 :\n");
    for j in data.type1.iter() {
        print!(" {} |", j);
    }
    println!("\n\nList of Type 2 :\n");
    for j in data.type2.iter() {
        print!(" {} |", j);
    }

    println!("\n\nList of Type 3 :\n");
    for j in data.type3.iter() {
        print!(" {} |", j);
    }
    println!("\n\nList of Type 4 :\n");
    for j in data.type4.iter() {
        print!(" {} |", j);
    }
}

pub fn unique(a: Vec<u64>) -> Vec<u64> {
    let mut x = a.clone();
    x.sort();
    x.dedup();
    return x;
}

