use general::{ejecutar_comando, leer_comando, map_comando, parse_input};

mod comandos;
mod errores;
mod general;

fn main() {
    loop {
        let input = leer_comando();
        if input == "salir" {
            break;
        }

        if let Some((comandos, args)) = parse_input(&input) {
            if let Some(comando) = map_comando(comandos, args) {
                ejecutar_comando(comando);
            }
        } else {
            println!("Error al emparejar el comando");
        }
    }
}
