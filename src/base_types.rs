use std::fmt;
use std::ops::{Add, AddAssign};
// , Sub



pub struct Value{
    pub x:i64,
    pub y:i64
}

impl fmt::Display for Value {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}-{}", self.x, self.y)
    }
}

impl Add<Value> for Value{
    type Output = Value;

    fn add(self, other: Value) -> Value {

        let added = self.x.checked_add(other.x);
        match added {
            Some(_x) => Value { x: self.x + other.x,y:self.y+other.y},
            None =>  {
                let rest = std::i64::MAX - self.x;
                let unit = other.x - rest;
                Value {x: unit,y:self.y+other.y+1} 
            },
        }
        
    }
    
}
impl Add<i64> for Value{
    type Output = Value;
    //TODO: Finish the add i64 to a value
    fn add(self, other: i64) -> Value {
        let added = self.x.checked_add(other);
        match added {
            Some(_x) => Value {x:self.x+other,y:self.y},
            None => {
                let rest = std::i64::MAX - self
            }
        }
        if added < self.x
        {
            Value {x: self.x + other, y: self.y +1}
        }
        else
        {
            Value {x: self.x + other, y: self.y}
        }
    }
    
    
}
impl AddAssign<i64> for Value{
    

    fn add_assign(&mut self, other: i64){
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