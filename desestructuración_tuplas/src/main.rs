fn main() {
   let tup = (234, 3.2, 'A');

   // Patrón de desestructuración
   let (x, y, z) = tup;

   println!("El valor de x es: {x}");
   println!("El valor de y es: {y}");
   println!("El valor de z es: {z}");

   let tup2: (i32, f64, char) = (5234, 3.2, 'A');

   // Acceder a los elemnentos de una tupla mediante 
   // el uso del operador punto y su índice numérico.
   let distancia = tup2.0;
   let velocidad = tup2.1;
   let clave = tup2.2;

   println!("El valor de distancia es: {distancia}");
   println!("El valor de velocidad es: {velocidad}");
   println!("El valor de clave es: {clave}");
}
