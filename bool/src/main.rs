fn main() {
   let t = true;
   // Con anotación explícita de tipo
   let f: bool = false;

   // Imprimir el valor de las variables booleanas
   println!("El valor de t es: {}", t);
   println!("El valor de f es: {}", f);

   // Uso de booleanos en una condición
   if t {
    println!("t es verdadero");
   } else {
    println!("t es falso");
   }

   if f {
    println!("f es verdadero");
   } else {
    println!("f es falso");
   }
}
