mod pila;
mod precedencia;
mod reglas;
mod shunting_yard;
mod nodo;

use std::fs;
use nodo::Nodo;

fn main() {
    println!("============================================");
    println!(" Conversión Infix a Postfix - Shunting Yard ");
    println!("============================================");


    let nodo_a = Nodo::operando("a".to_string());
    let nodo_b = Nodo::operando("b".to_string());

    let nodo_or = Nodo::binario(
        "|".to_string(),
        nodo_a,
        nodo_b,
    );

    println!("{:#?}", nodo_or);

    let contenido = match fs::read_to_string("expresionesRegulares.txt") {
        Ok(contenido) => contenido,

        Err(error) => {
            eprintln!(
                "No fue posible abrir expresionesRegulares.txt: {}",
                error
            );

            return;
        }
    };

    for (indice, linea) in contenido.lines().enumerate() {
        let expresion = linea.trim();

        if expresion.is_empty() {
            continue;
        }

        println!("\n============================================");
        println!("Expresión número {}", indice + 1);
        println!("============================================");

        match shunting_yard::convertir_a_postfix(expresion) {
            Ok(resultado) => {
                println!("\nResultado:");
                println!(
                    "Postfix antes de convertir + y ?: {}",
                    resultado.postfix_original
                );

                println!(
                    "Postfix final: {}",
                    resultado.postfix_convertido
                );
            }

            Err(error) => {
                println!("\nError: {}", error);
            }
        }
    }
}