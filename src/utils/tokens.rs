
/**
 * Represents the various token types that can be encountered during the lexical analysis 
 * of a programming language. Each token type serves as a category for identifying 
 * different syntactic elements in the source code.
 *
 * ## Variants
 *
 * ### Literals
 * - `StringLiteral`: A token representing string values (e.g., `"Hello, world!"`).
 * - `NumberLiteral`: A token representing numeric values (e.g., `42`, `3.14`).
 * - `BooleanLiteral`: A token representing boolean values (e.g., `true`, `false`).
 *
 * ### Identifiers
 * - `Identifier`: A token representing variable or function names.
 *
 * ### Keywords - English
 * Tokens representing reserved keywords in the English version of the language:
 * - `Moo`, `Text`, `Num`, `Dec`, `Boolean`, `Coop`, `BarnMap`: Data type declarations and constructs.
 * - `If`, `ElseIf`, `Else`: Conditional constructs.
 * - `While`: A loop construct.
 * - `For`, `In`: Loop constructs with iteration.
 * - `Barn`: Represents a specific construct or block (e.g., a scoped area).
 * - `Farmfunction`: Defines a function in the language.
 * - `Return`: Indicates a return statement.
 * - `Is`, `IsNot`: Indicates equality or inequality checks.
 *
 * ### Keywords - Spanish
 * Tokens representing reserved keywords in the Spanish version of the language:
 * - `Muuu`, `Texto`, `Numero`, `Decimal`, `Booleano`, `Granja`, `MapaEstablo`: Corresponding data type declarations and constructs in Spanish.
 * - `Si`, `SinoSi`, `Sino`: Conditional constructs in Spanish.
 * - `Mientras`: Loop construct in Spanish.
 * - `Para`, `En`: Loop constructs with iteration in Spanish.
 * - `Granero`: Corresponds to the English keyword `Barn`.
 * - `Funciongranja`: Corresponds to the English keyword `Farmfunction`.
 * - `Regresa`: Corresponds to the English keyword `Return`.
 * - `Es`, `NoEs`: Corresponds to `Is` and `IsNot`.
 *
 * ### Operators
 * Tokens representing mathematical or logical operations:
 * - `Plus`: Addition operator (`+`).
 * - `Minus`: Subtraction operator (`-`).
 * - `Multiply`: Multiplication operator (`*`).
 * - `Divide`: Division operator (`/`).
 * - `IntegerDivide`: Integer division operator (`//`).
 * - `Modulo`: Modulo operator (`%`).
 * - `LessThan`: Less-than comparison operator (`<`).
 * - `GreaterThan`: Greater-than comparison operator (`>`).
 * - `Increment`: Increments a value (`++`).
 * - `Decrement`: Decrements a value (`--`).
 *
 * ### Punctuation
 * Tokens representing structural elements or delimiters in the syntax:
 * - `LeftParen`, `RightParen`: Parentheses (`(` and `)`).
 * - `LeftBrace`, `RightBrace`: Curly braces (`{` and `}`).
 * - `LeftBracket`, `RightBracket`: Square brackets (`[` and `]`).
 * - `Comma`: Comma (``,`).
 * - `Dot`: Dot (`.`).
 * - `Colon`: Colon (`:`).
 * - `Semicolon`: Semicolon (`;`).
 * - `Quote`: Quotation mark (`"`).
 * - `Equals`: Equals sign (`=`).
 *
 * ### Special Tokens
 * Tokens that represent specific, specialized elements:
 * - `Newline`: Represents the end of a line in the source code.
 * - `EOF`: Represents the end of the source file.
 * - `Comment`: Represents a comment in the source code.
 *
 * ## Traits
 * - `Debug`: Allows for formatted debugging output for the `TokenType` enum.
 * - `Clone`: Enables cloning of `TokenType` instances.
 * - `PartialEq`: Enables comparison of `TokenType` instances for equality.
 */
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

//TODO: Plan token structure and methods
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

