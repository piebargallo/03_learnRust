use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("¡adivina un número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);
    
    let mut intentos = 0;

    loop {
        println!("Por favor, introduce tu número.");

        let mut adivina = String::new();

        io::stdin()
            .read_line(&mut adivina)
            .expect("No se puede leer la línea");
    // En rust, cuando se hace shadowing(sombreado) de una variable, se 
    // está creando un nuevo binding con el mismo nombre, pero 
    // potencial... con un tipo diferente o distintas reglas. Eso
    // significa que la anterior var queda eclipsada y, a todos los
    // efectos, ya no se puede utilizar.
        let adivina: u32 = match adivina.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! Por favor, intro un número válido.\n");
                continue;
            }
        };
            
        intentos += 1;

        match adivina.cmp(&numero_secreto) {
            Ordering::Less => println!("¡Muy pequeño!"),
            Ordering::Greater => println!("¡Muy grande!"),
            Ordering::Equal => {
                println!("¡Has acertado");
                println!("Lo has logrado en {intentos} intentos.");
                break;
            }
        }
    }
}
