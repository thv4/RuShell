use std::fs;

pub fn cmd_br(args: &[String]) {
    if args.is_empty() {
        eprintln!("Uso: br <archivo>");
        return;
    }
    match fs::remove_file(&args[0]) {
        Ok(_) => println!("El archivo {} ha sido eliminado", args[0]),
        Err(_) => eprintln!("Error al eliminar {}", args[0]),
    }
}
