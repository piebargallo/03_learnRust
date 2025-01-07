fn main() {
    // Para ilustrar el overflow, usaremos tipos de 8 bits (u8).
    let a: u8 = 200;
    let b: u8 = 100;

    println!("a = {}, b = {}", a, b);

    // 1) wrapping_add
    // Si hay overflow, 'envuelve' (wrap around) dentro del rango del tipo u8
    // Es como una operación modular: (200 + 100) mod 256 = 44
    let wrapped_sum = a.wrapping_add(b);
    println!("1) wrapping_add => {}", wrapped_sum);

    // 2) checked_add
    // Devuelve Some(resultado) si no hay overflow, o None si lo hay
    match a.checked_add(b) {
        Some(result) => println!("2) checked_add => {}", result),
        None => println!("2) checked_add => Overflow (None)"),
    }

    // 3) overflowing_add
    // Devuelve una tupla (resultado, bool)
    // donde bool indica si se produjo overflow
    let (overflow_result, did_overflow) = a.overflowing_add(b);
    println!(
        "3) overflowing_add => result: {}, did_overflow: {}",
        overflow_result, did_overflow
    );

    // 4) saturing_add
    // Si hay overflow, toma el valor máximo del tipo (u8::MAX = 255)
    let saturated_sum = a.saturating_add(b);
    println!("4) saturating_add => {}", saturated_sum);
}
