pub fn cmd_aqui(_args: &[String]) {
    let path = std::env::current_dir().unwrap();
    println!("{}", path.display());
}
