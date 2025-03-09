fn main(){
    let (mut x, mut y, mut z) = (1, 2, 3);
    fn comeguie<'a>(x: &'a mut i32)-> &mut i32 {*x+=3; x }
let result=        comeguie(&mut x);
println!("{}", result);
}