
use compiler::toml_config::language_config;
use std::path::Path;

fn main() {
    let path = Path::new("/home/batman/Documents/proyects/compiler/src/moo_lang.toml");
    let config = language_config::MooConfig::from_toml(&path);
    println!("{:?}", config);

}
