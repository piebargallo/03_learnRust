const DIAS_SEMANA: u32 = 7;
const HORAS_DIA: u32 = 24;
// Asignación constante una expresión constante.
const HORAS_SEMANA: u32 = DIAS_SEMANA * HORAS_DIA;

fn main() {
    println!("Una semana tiene {} horas.", HORAS_SEMANA);    
}
