#[derive(Debug)]
pub enum Color {
    Red ,
    Green ,
    Blue ,
    Orange ,
    RGB(u8,u8,u8) 
}

pub fn main(){
    let red : Color = Color::Red ;
    print!("{:?}\n" , red ) ;
}