use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Advinhe o numero");

    let segredo = rand::thread_rng().gen_range(1..=100);


    loop{

        println!("Por favor, insira sua advinha: ");

        let mut advinha = String::new();

        io::stdin()
            .read_line(&mut advinha)
            .expect("falha ao ler");

        println!("Voce advinhou: {}", advinha);

        let advinha: u32 = match advinha.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match advinha.cmp(&segredo) {
            Ordering::Less => println!("Abaixo!"),
            Ordering::Greater => println!("Acima"),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }

        
    }
}

// mut indica uma variável mutável
// a ausência de mut implica uma variável imutável; em Rust, as variáveis são imutáveis por padrão
