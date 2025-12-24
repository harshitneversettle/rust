#[derive(Debug)]
pub struct Dimensions {
    length : u64 ,
    width : u64 ,
}

impl Dimensions {
    pub fn set(len : u64 , wid : u64 ) -> Self{
        let shape1 = Self { length : len , width : wid } ;
        return shape1 ;
    }
    
    pub fn area (&self)->Result<u64 , &'static str>{
        let ans = self.length.checked_mul(self.width).expect("something is 0");
        Ok(ans)
    }
}

pub fn main(){
    let rect : Dimensions = Dimensions::set(10, 20) ;
    let area : u64 = Dimensions::area(&rect).unwrap() ;
    print!("{:?}\n" , rect ) ;
    print!("{:?}\n" , area ) ;
}