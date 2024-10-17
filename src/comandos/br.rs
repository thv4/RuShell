use std::fs;

pub fn cmd_br(destino: &str) {
    match fs::remove_file(destino) {
        Ok(_) => println!("El archivo {} ha sido eliminado", destino),
        Err(_) => eprintln!("Error al eliminar {}", destino),
    }
}
