pub fn cmd_aqui() {
    let path = std::env::current_dir().unwrap();
    println!("{}", path.display());
}
