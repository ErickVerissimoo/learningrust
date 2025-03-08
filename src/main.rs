use meumod::compiuter;

mod meumod;
pub fn main() {
    let mut nome: &str="erick";
    println!("meu nome é {}", nome);
    nome = "e";
    println!("novo nome {}", nome);

    let mut vecto= vec![];
    
    vecto.push(String::from("comequie"));

let p = Pessoa("Olá".to_string(), 18);
println!("nome é {}", p.0);    
println! ("a soma é {} ", soma(2, 3));
if vecto.contains(&"comequie".to_string()){
println!("contem");
}else{
    println!("não contem");
}
let car = carro{
    marca: "ford".to_string(),
    lancamento: 2000
};
println!("a marca é {}", car.marca);
let _comp = compiuter{
    giga: "comequie".to_string()
};
}
pub struct Pessoa(String, i32);
impl Pessoa{
    fn ola()->String{
        return "ola".to_string();
    }
}
pub struct carro{
    marca: String,
    lancamento: i32
}
fn soma(um: i32, dois:i32)->i32{
 um+dois
}