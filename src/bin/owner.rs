fn main(){
let variave: String=String::from("comeguie");    
let mut ugue = &variave;
/*
O código abaixo não compila pois o valor de variave
foi integralmente transferido para ugue deixando variave vazio
*/
// println!("ugue {}", variave);

let  mut ugue:Option<i32> = Option::None;
ugue = Option::from(2);
let re;
{
   let r=String::from("ugue disse");
    re= &r;
    println!("ola {}", re);
    let mut variaver: i32=3;
    println!("o valor é {}", variaver);
    uguedisse(&mut variaver);
    println!("o valor é {}", variaver);

    let mut ugue =3;
    let mut teste = &ugue;

    
}
}

fn uguedisse(ugue: & mut i32) {

    *ugue=23
}

