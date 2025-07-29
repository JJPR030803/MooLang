#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    
    // Identifiers
    Identifier,
    
    // Keywords - English
    Moo, Text, Num, Dec, Boolean, Coop, BarnMap,
    If, ElseIf, Else, While, For, In, Barn,
    Farmfunction, Return, Is, IsNot,
    
    // Keywords - Spanish  
    Muuu, Texto, Numero, Decimal, Booleano, Granja, MapaEstablo,
    Si, SinoSi, Sino, Mientras, Para, En, Granero,
    Funciongranja, Regresa, Es, NoEs,
    
    // Operators
    Plus, Minus, Multiply, Divide, IntegerDivide, Modulo,
    LessThan, GreaterThan, Increment, Decrement,
    
    // Punctuation
    LeftParen, RightParen, LeftBrace, RightBrace,
    LeftBracket, RightBracket, Comma, Dot, Colon,
    Semicolon, Quote, Equals,
    
    // Special
    Newline, EOF, Comment,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}