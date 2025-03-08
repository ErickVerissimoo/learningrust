fn main(){
    let pe = Pessoa{
        nome: "Erick".to_string(),
        idade: 19
    };
    println!("ola {} de idade {}", pe.nome, pe.idade);
}

struct Pessoa{
    nome: String,
    idade: i32
}