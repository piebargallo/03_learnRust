fn main() {
    // Rust realiza inferencia de tipos y aplica de modo
    // predeterminado "f64".
    //let x = 2.0; // f64

    // Para usar "f32" debemos hacer una anotación 
    // explícita de tipo.
    //let y: f32 = 3.0; // f32

    // El valor devuelto por la división será un entero
    let cociente_truncado = (-5.3_f64 / 3.2_f64).trunc();
    println!(
        "El coeficiente truncado de -5.3 dividido por 3.2 es: {}",
        cociente_truncado
    );
}
