fn main(){
    let mut x =3;
    let mut y =&mut x;
    *y=45;
    println!("o valor é {}", x);
   let mut testage= 3;
   soma(&mut testage);
   println!("{}", testage);

}

fn soma(x: & mut i32){
   *x=*x+3

}