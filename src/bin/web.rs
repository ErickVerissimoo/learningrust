use std::io;
fn main(){
    let mut input = String::new();
    println!("entre com seu nome");
    io::stdin().read_line(&mut input)
    .expect("falha ao ler a entrada");
    panic!("panico");


    println!("olá {}", input);
}

fn comequie(){
    let func = |nome: &str|{
        println!("olá {}",nome);
    };
    func("erick");
    
}

struct pessoa <'a> {
    nome: &'a str,
    idade: i32

}