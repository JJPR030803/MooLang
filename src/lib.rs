pub mod lexer;
pub mod utils{
    pub mod file_reader;
    pub mod file_reader_errors;
    pub mod tokens;
}
pub mod toml_config{
    pub mod language_config;
    pub mod config_errors;
}
//pub mod parser;