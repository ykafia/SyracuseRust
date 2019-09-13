use std::fmt;
use std::ops::{Add, Sub, AddAssign};




pub struct Value{
    pub x:u64,
    pub y:u64
}

impl fmt::Display for Value {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{}-{}", self.x, self.y)
    }
}


pub struct SyracuseInfo{
    pub flight:u64,
    pub height:u64,
    pub above_flight:u64,
    pub updated_above:bool,
    pub current:u64,
    pub init:u64
}

impl fmt::Display for SyracuseInfo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "Flight : {}\nHeight : {}\nAbove Flight : {}\n", self.flight, self.height,self.above_flight)
    }
}



impl Add<Value> for Value{
    type Output = Value;

    fn add(self, other: Value) -> Value {

        let added = self.x.checked_add(other.x);
        match added {
            Some(_x) => Value { x: self.x + other.x,y:self.y+other.y},
            None =>  {
                let rest = std::u64::MAX - self.x;
                let unit = other.x - rest;
                Value {x: unit,y:self.y+other.y+1} 
            },
        }
        
    }
    
}

impl Sub<Value> for Value{
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        let added = self.x.checked_sub(other.x);
        match added {
            Some(_x) => Value { x: self.x - other.x,y:self.y-other.y},
            None =>  {
                let rest = other.x - self.x;
                let unit = std::u64::MAX - rest;
                Value {x: unit,y:self.y-other.y-1} 
            },
        }
    }
}


impl Add<u64> for Value{
    type Output = Value;
    fn add(self, other: u64) -> Value {
        let added = self.x.checked_add(other);
        match added {
            Some(_x) => Value {x:self.x+other,y:self.y},
            None => {
                let rest = std::u64::MAX - self.x;
                let unit = other - rest;
                Value {x:unit,y:self.y+1}
            }
        }
        
    }
}

impl Sub<u64> for Value{
    type Output = Value;
    fn sub(self, other: u64) -> Value {
        let added = self.x.checked_add(other);
        match added {
            Some(_x) => Value {x:self.x-other,y:self.y},
            None => {
                let rest = other - self.x;
                let unit = std::u64::MAX - rest;
                Value {x:unit,y:self.y-1}
            }
        }
        
    }
}
impl AddAssign<u64> for Value{
    

    fn add_assign(&mut self, other: u64){
        let added = self.x + other;
        if added < self.x
        {
           self.x = self.x+other;
           self.y = self.y+1;
        }
        else
        {
            self.x = self.x + other;
        }
    }
    
    
}