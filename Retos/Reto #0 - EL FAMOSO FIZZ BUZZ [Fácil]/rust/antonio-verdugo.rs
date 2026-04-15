fn main() {
    // /*
    //  * Escribe un programa que muestre por consola (con un print) los
    //  * números de 1 a 100 (ambos incluidos y con un salto de línea entre
    //  * cada impresión), sustituyendo los siguientes:
    //  * - Múltiplos de 3 por la palabra "fizz".
    //  * - Múltiplos de 5 por la palabra "buzz".
    //  * - Múltiplos de 3 y de 5 a la vez por la palabra "fizzbuzz".
    //  */
    for num in 1..101 {
        let mut result = String::new();
        if num % 3 == 0 {
            result.push_str("fizz");
        }
        if num % 5 == 0 {
            result.push_str("buzz");
        }
        if !result.is_empty() {
            println!("{result}");
            continue;
        }
        println!("{num}");
    }
}
