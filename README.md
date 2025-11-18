# MooLang - Multilingual Educational Programming Language

![Status](https://img.shields.io/badge/status-in%20development-yellow)
![Language](https://img.shields.io/badge/rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-green)

> A farm-themed programming language compiler that bridges the gap between block-based and traditional text-based programming through multilingual keyword support.

## Overview

MooLang is an educational programming language designed to help students transition from visual programming environments (like Scratch) to traditional text-based languages. The project's unique approach allows educators to teach programming concepts in **any natural language** through a configurable TOML-based keyword system.

The compiler is built in Rust and currently implements a **lexical analyzer with dynamic keyword mapping**. What makes MooLang distinctive is its ability to support multiple natural languages (English, Spanish, German, Russian, and custom variants) while maintaining identical underlying semantics—a single codebase can be written in different human languages with perfect interoperability.

**Current Focus:** Lexer implementation with TOML-driven configuration system
**Target Audience:** Computer science educators and students aged 12-18

## Features / What's Implemented ✅

### Fully Functional
- **TOML-Based Language Configuration System**
  - Load keyword mappings from external `.toml` files
  - Smart defaults with partial config merging
  - Supports 4+ natural languages out-of-the-box (English, Spanish, German, Russian)
  - Custom keyword variants for classroom customization

- **Dynamic Keyword Manager**
  - Runtime keyword-to-token mapping
  - Language-specific token resolution
  - Support for multi-word keywords (`"else if"`, `"is not"`)

- **File Type System**
  - `.moo` extension for English-based programs
  - `.muu` extension for Spanish-based programs
  - Language detection from file extension

- **Token System Foundation**
  - 60+ token types defined (literals, operators, keywords, punctuation)
  - Dual-language keyword support (English/Spanish tokens)
  - Position tracking (line, column) ready for error reporting

### In Development 🚧
- **Lexer Implementation** (partially complete)
  - Tokenization logic structure defined
  - Integration with keyword manager in progress

### Planned 📋
- Parser (AST generation)
- Semantic analyzer
- Interpreter/bytecode generator
- Standard library with farm-themed functions
- IDE plugin with syntax highlighting

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    MooLang Compiler                      │
├─────────────────────────────────────────────────────────┤
│  Source Code (.moo/.muu) ──► File Reader                │
│                              │                           │
│                              ▼                           │
│  TOML Config ──────────► Language Manager               │
│  (moo_lang.toml)            │                           │
│                              ▼                           │
│                          Lexer [WIP]                     │
│                              │                           │
│                              ▼                           │
│                          Parser [Planned]                │
│                              │                           │
│                              ▼                           │
│                       Interpreter [Planned]              │
└─────────────────────────────────────────────────────────┘
```

**Key Design Decision:** Separation of language syntax from compiler logic. The `LanguageKeywordManager` dynamically builds token mappings at runtime, allowing teachers to create custom language variants without modifying source code.

## Technical Stack

- **Language:** Rust 2021 Edition
- **Configuration:** TOML (via `toml` crate v0.9.5)
- **Serialization:** Serde v1.0.219
- **Build System:** Cargo

## Quick Start

### Prerequisites
- Rust 1.70+ (2021 edition support)
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/MooLang.git
cd MooLang

# Build the project
cargo build --release

# Run tests
cargo test
```

### Basic Usage

**Example MooLang program (English - `.moo`):**
```moo
# Farm inventory management
coop 'text' animals = ["cow", "chicken", "pig"]
coop 'num' counts = [5, 10, 3]

farmfunction addAnimal(text animal, num count) {
    animals.plant(animal)
    counts.plant(count)
    moo "Added", count, animal, "to the farm"
}

addAnimal("sheep", 4)
```

**Same program in Spanish (`.muu`):**
```muu
# Gestión de inventario de granja
granja 'texto' animales = ["vaca", "pollo", "cerdo"]
granja 'numero' cantidades = [5, 10, 3]

funciongranja agregarAnimal(texto animal, numero cantidad) {
    animales.plantar(animal)
    cantidades.plantar(cantidad)
    muuu "Agregado", cantidad, animal, "a la granja"
}

agregarAnimal("oveja", 4)
```

### Language Configuration

The compiler loads keyword mappings from `src/moo_lang.toml`:

```toml
[language]
version = "1.0.0"
default_language = "en"

[keywords.en]
print = "moo"
if_keyword = "if"
function_keyword = "farmfunction"
# ... 17 keyword mappings per language

[keywords.custom]  # Teachers can add custom variants
print = "say"
if_keyword = "when"
function_keyword = "procedure"
```

## Project Structure

```
MooLang/
├── src/
│   ├── main.rs                    # Entry point
│   ├── lib.rs                     # Module exports
│   ├── lexer/
│   │   ├── lexer.rs               # Tokenization logic [WIP]
│   │   └── errors.rs              # Lexer error types
│   ├── toml_config/
│   │   ├── language_config.rs     # Config loader + keyword manager
│   │   └── config_errors.rs       # Configuration error handling
│   ├── utils/
│   │   ├── tokens.rs              # Token type definitions
│   │   ├── file_reader.rs         # .moo/.muu file handler
│   │   └── file_reader_errors.rs  # File I/O errors
│   └── moo_lang.toml              # Default language keywords
├── docs/
│   ├── language_documentation_en.md  # Language spec (English)
│   └── language_documentation_es.md  # Language spec (Spanish)
└── Cargo.toml                     # Rust dependencies
```

## Technical Highlights

### 1. Configuration-Driven Multilingual Support
Unlike traditional compilers with hardcoded keywords, MooLang uses a TOML-based system that allows runtime language switching. This enables:
- **Educational flexibility:** Teachers can customize keywords for curriculum needs
- **Internationalization:** No code changes needed to add new languages
- **Accessibility:** Students code in their native language while learning universal concepts

**Implementation** (`src/toml_config/language_config.rs:72-115`):
```rust
pub struct LanguageKeywordManager {
    config: MooConfig,
    token_maps: HashMap<String, HashMap<String, TokenType>>,
}

impl LanguageKeywordManager {
    fn build_token_maps(&mut self) {
        for (lang, keyword_set) in &self.config.keywords {
            let mut token_map = HashMap::new();
            token_map.insert(keyword_set.print.clone(), self.get_print_token(lang));
            // Maps "moo" → TokenType::Moo for English
            // Maps "muuu" → TokenType::Muuu for Spanish
            self.token_maps.insert(lang.clone(), token_map);
        }
    }
}
```

### 2. Smart Configuration Merging
The system supports partial TOML configurations that merge with defaults, allowing teachers to override only specific keywords without redefining entire language sets (`src/toml_config/language_config.rs:283-310`).

### 3. Farm-Themed Syntax
All keywords use farm/animal terminology to create an engaging learning environment:
- `moo` (print), `farmfunction` (function), `coop` (array), `barn` (range)
- Spanish equivalents: `muuu`, `funciongranja`, `granja`, `granero`

## Current Status & Roadmap

### Completed ✅
- [x] TOML configuration system with language support
- [x] Token type definitions (60+ types)
- [x] Keyword manager with dynamic mapping
- [x] File type detection and reading
- [x] Language documentation (English & Spanish)

### In Progress 🚧
- [ ] Lexer implementation (structure defined, logic incomplete)
- [ ] Error reporting system

### Next Steps 📋
1. **Immediate:** Complete lexer tokenization logic
2. **Short-term:** Implement parser with AST generation
3. **Medium-term:** Add semantic analysis and type checking
4. **Long-term:** Build interpreter and create VS Code extension

### Known Limitations
- Lexer not yet functional (tokenization logic incomplete)
- No parser/interpreter (design planned, implementation pending)
- Limited to 4 predefined languages + custom (extensible by design)

## What This Project Demonstrates

**Technical Skills:**
- Rust systems programming (ownership, lifetimes, pattern matching)
- Compiler theory (lexical analysis, token design)
- Configuration-driven architecture
- Multilingual software design
- Error handling with custom types

**Software Engineering:**
- Separation of concerns (modular architecture)
- Extensibility through configuration
- Documentation-first development
- Test-driven development setup

**Educational Technology:**
- Understanding of pedagogy in CS education
- Accessibility through internationalization
- User-centered design for educators

## Vision & Educational Context

MooLang emerged from observing the steep learning curve students face when transitioning from Scratch to languages like Java or Python. The farm theme reduces intimidation while the multilingual support addresses language barriers in programming education.

**Inspiration:** Bridging the gap between Logo/Scratch (visual) and Python (text-based) with explicit educational scaffolding.

**Differentiation from Academic Projects:**
- Extended beyond typical "build a lexer" assignments to include production-ready configuration system
- Real-world consideration of internationalization from day one
- Designed for actual use in educational settings, not just as a learning exercise

## Contributing

This is an active educational project. Contributions welcome in:
- Additional language translations (French, Chinese, Arabic, etc.)
- Lexer/parser implementation
- Documentation improvements
- Test coverage expansion

## License

MIT License - see LICENSE file for details

---

**Note:** This project is under active development. The lexer is currently being implemented, and the parser/interpreter are in the design phase. The configuration system and architecture are production-ready, demonstrating a solid foundation for the complete compiler.

**Contact:** For questions about MooLang's educational applications or technical architecture, feel free to open an issue.

---

*Documentation available in:* [English](docs/language_documentation_en.md) | [Español](docs/language_documentation_es.md)
