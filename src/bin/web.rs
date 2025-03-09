use std::io;
fn main(){
    let mut input = String::new();
    println!("entre com seu nome");
    io::stdin().read_line(&mut input)
    .expect("falha ao ler a entrada");
    


    println!("olá {}", input);

    for i in 1..=10{
        println!("{}", i);
    }

}

fn comequie(){
    let func = |nome: &str|{
        println!("olá {}",nome);
    };
    func("erick");
 fn testar(){
    
 }   
}

struct pessoa <'a> {
    nome: &'a str,
    idade: i32

}