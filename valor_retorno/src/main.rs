// función retorno implícita
fn suma(a: i32, b: i32) -> i32 {
    a + b
}

// función retorno explícita
fn suma2(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let res1 = suma(2, 2);
    println!("El resultado implícito es: {}", res1);

    let res2 = suma2(1, 1);
    println!("El resultado explícito es: {}", res2);
}
