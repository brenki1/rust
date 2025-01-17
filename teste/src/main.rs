use std::{f64::consts::PI, io};

fn main(){

    let valor_absoluto: f64 = 0.00001;

    let mut valor_graus: f64 = 0.0;
    let mut texto = String::new();
    
    println!("Calculo de cosseno");
    println!("Insira o valor em graus para ser calculado seu cosseno: ");

    io::stdin()
        .read_line(&mut texto)
        .expect("Falha ao ler");

    valor_graus = texto.trim().parse().expect("Nao eh um numero valido");
    let mut radianos = graus_para_radiano(valor_graus);
    
    let mut iteracao: f64 = 0.0;

    while radianos > valor_absoluto {
        radianos += ((f64::powf(-1.0,iteracao))/(fatorial(2.0*iteracao))) * f64::powf(radianos,2.0*iteracao);
        iteracao += 1.0;
    }

    println!("O cosseno do angulo inserido eh: {radianos}");

}

fn graus_para_radiano(x: f64) -> f64 {
    x * (PI/180.0)
}

fn fatorial(x: f64) -> f64 {

    if x <= 1.0 {
        return 1.0;
    }

    return x * fatorial(x-1.0);
        
}