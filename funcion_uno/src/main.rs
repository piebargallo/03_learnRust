fn main() {
    println!("Estamos en la función main");

    // Llamada a la función
    segunda_funcion();

    // 4 es el argumento
    tercera_funcion(4);

    medidas(4, 'h');

    // Declaración
    let resultado = suma(5, 6);
    println!("El resultado es: {}", resultado);
}

// Firma de la función
fn segunda_funcion() {
    // Cuerpo de la función
    println!("Estamos en la segunda función.");
}

// x es el parámetro, rust requiere anotaciones de tipo
// en las definiones de funciones
fn tercera_funcion(x: i32) {
    println!("El valor de x es: {x}");
}

// Notación explícita de los parámetros
fn medidas(valor: i32, etiqueta: char) {
    println!("La medida es: {valor}{etiqueta}");
}

fn suma(a: i32, b: i32) -> i32 {
    // Expresión autónoma, retorno implícito
    a + b
}
