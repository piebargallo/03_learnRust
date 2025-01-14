fn main() {
    // Inferencia completa
    let tup1 = (10, "Hello", true);
    println!("{:?}", tup1);

    // Tipo completo anotado, rust solo permite anotación de
    // tipos en tuplas para todos los tipos, no solo para 
    // algunos. O todos o ninguno.
    let tup2: (i32, f64, char) = (5234, 3.2, 'A');
    println!("{:?}", tup2);
}
