use std::io;

fn main() {
    // 1. El compilador usa inferencia de tipos y tamaño
    let array_inferencia_completa = [1, 2, 3, 4, 5];
    println!(
        "Array con inferencia de tipos y tamaño: {:?}",
        array_inferencia_completa
    );

    // 2. Usamos anotación tanto de tipos como de tamaño
    let array_anotacion_completa: [i32; 5] = [10, 20, 30, 40, 50];
    println!(
        "Array co anotación de tipo y tamaño: {:?}",
        array_anotacion_completa
    );

    // 3. Anotamos el tamaño y el compilador infiere tipo, guión bajo
    // cumple el papel de comodin o playholder
    let array_inferencia_tamano: [_; 4] = [1.1, 2.2, 3.3, 4.4];
    println!(
        "Array con anotación de tipo e inferencia de tamaño: {:?}",
        array_inferencia_tamano
    );

    let a = [1, 2, 3, 4];
    /*Acceso de los elementos con indexación
    let primero = a[0];
    println!("El primer elemento es: {}", primero);*/

    println!("Escribe un índice de array: ");

    let mut indice = String::new();

    io::stdin()
        .read_line(&mut indice)
        .expect("No se ha podido leer la línea");

    let indice: usize = indice
        .trim()
        .parse()
        .expect("El índice escrito no es un número");

    let elemento = a[indice];

    println!("El valor del elemento en el índice {indice} es: {elemento}");
}
