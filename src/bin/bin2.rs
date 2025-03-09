use std::fmt::Debug;

pub fn main(){
   let func = |nome: &str|{
    println!("Olá {}", nome)
   };
   func("erick");
let mut tipe:Vec<&str> = Vec::new();
tipe.push("comeguie");

fn ugue<T: Debug>(value: T){
println!(" o valor é: {:#?}", value);
}

let  pessoa: person = person{
   nome: "erick",
};

ugue(pessoa);

}
#[derive(Debug)]
struct person<'a> {
   nome: &'a str,
   
}


