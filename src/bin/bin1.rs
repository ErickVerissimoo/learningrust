#[derive(Debug)]
struct Pessoa{
    nome: String,
    idade: i32
    
}

impl Pessoa{
 fn ola(&mut self,  nome: &str ) {
    self.idade=23;
    self.nome=String::from(nome)
}

}
impl ParaPessoa for Pessoa{
     fn hello(&self) -> (){
        println!("oi");
     }
}

fn main(){
    let mut  pe:Pessoa = Pessoa{
        nome: "Erick".to_string(),
        idade: 19
    };
    println!("ola {} de idade {}", pe.nome, pe.idade);
        pe.ola("comeguie");
        println!("oi {:?}", pe);
    pe.hello();
    }
    trait ParaPessoa{
        fn hello(&self) -> ();
    }


