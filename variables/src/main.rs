fn main() {
    // El valor predeterminado es mutable, no son constantes.
    let x = 4;
    println!("El valor de x es: {x}");

    let mut y = 1;
    println!("El valor de y es: {y}");
    y = 2;
    println!("El valor de y es: {y}");

    // Var inmutable
    let z = 10;
    println!("z es inmutable: {}", z);

    // Ensombrecemos 'z', ahora es mutable
    let mut z = z + 2;
    println!("z ahora es mutable y vale: {}", z);

    // Modificamos el valor de la nueva 'z'
    z += 2;
    println!("z después de la modificación: {}", z);
}
