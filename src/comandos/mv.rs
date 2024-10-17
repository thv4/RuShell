use std::fs;

pub fn cmd_mv(origen: &str, destino: &str) {
    match fs::rename(origen, destino) {
        Ok(_) => println!("Archivo movido de {} a {}", origen, destino),
        Err(_) => eprintln!("Error al mover el archivo de {} a {}", origen, destino),
    }
}
