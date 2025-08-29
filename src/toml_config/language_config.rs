use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt, fs};
use std::path::Path;
use std::fmt::{Display, Formatter};
use crate::toml_config::config_errors::ConfigError;
use crate::utils::tokens::TokenType;

// ================================
// Configuration Structures
// ================================

/// Main configuration struct for MooLang compiler
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MooConfig {
    pub language: LanguageSettings,
    pub keywords: HashMap<String, KeyWordSet>,
}

/// Language settings and metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageSettings {
    pub version: String,
    pub default_language: String,
}

/// Complete keyword set for a language
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyWordSet {
    // Output
    pub print: String,

    // Data types
    pub text_type: String,
    pub number_type: String,
    pub decimal_type: String,
    pub boolean_type: String,
    pub array_type: String,
    pub map_type: String,

    // Control flow
    pub if_keyword: String,
    pub else_if_keyword: String,
    pub else_keyword: String,
    pub while_keyword: String,
    pub for_keyword: String,
    pub in_keyword: String,

    // Functions
    pub function_keyword: String,
    pub return_keyword: String,

    // Comparison
    pub is_keyword: String,
    pub is_not_keyword: String,

    // Special
    pub range_keyword: String,
}

// ================================
// Runtime Keyword Manager
// ================================

/// Dynamic keyword manager that converts config to TokenType mappings
pub struct LanguageKeywordManager {
    config: MooConfig,
    token_maps: HashMap<String, HashMap<String, TokenType>>,
}

impl LanguageKeywordManager {
    /// Create from loaded config
    pub fn from_config(config: MooConfig) -> Self {
        let mut manager = Self {
            config,
            token_maps: HashMap::new(),
        };
        manager.build_token_maps();
        manager
    }

    /// Create from config file with smart defaults
    pub fn from_file(path: &Path) -> Result<Self, ConfigError> {
        let config = MooConfig::load_with_defaults(path)?;
        Ok(Self::from_config(config))
    }

    /// Build token mappings from keyword sets
    fn build_token_maps(&mut self) {
        for (lang, keyword_set) in &self.config.keywords {
            let mut token_map = HashMap::new();

            // Map keywords to their TokenTypes
            token_map.insert(keyword_set.print.clone(), self.get_print_token(lang));
            token_map.insert(keyword_set.text_type.clone(), TokenType::Text);
            token_map.insert(keyword_set.number_type.clone(), TokenType::Num);
            token_map.insert(keyword_set.decimal_type.clone(), TokenType::Dec);
            token_map.insert(keyword_set.boolean_type.clone(), TokenType::Boolean);
            token_map.insert(keyword_set.array_type.clone(), self.get_array_token(lang));
            token_map.insert(keyword_set.map_type.clone(), self.get_map_token(lang));
            token_map.insert(keyword_set.if_keyword.clone(), TokenType::If);
            token_map.insert(keyword_set.else_if_keyword.clone(), self.get_else_if_token(lang));
            token_map.insert(keyword_set.else_keyword.clone(), TokenType::Else);
            token_map.insert(keyword_set.while_keyword.clone(), self.get_while_token(lang));
            token_map.insert(keyword_set.for_keyword.clone(), self.get_for_token(lang));
            token_map.insert(keyword_set.in_keyword.clone(), self.get_in_token(lang));
            token_map.insert(keyword_set.function_keyword.clone(), self.get_function_token(lang));
            token_map.insert(keyword_set.return_keyword.clone(), self.get_return_token(lang));
            token_map.insert(keyword_set.is_keyword.clone(), self.get_is_token(lang));
            token_map.insert(keyword_set.is_not_keyword.clone(), self.get_is_not_token(lang));
            token_map.insert(keyword_set.range_keyword.clone(), self.get_range_token(lang));

            self.token_maps.insert(lang.clone(), token_map);
        }
    }

    /// Get token mappings for a specific language
    pub fn get_token_map(&self, language: &str) -> Option<&HashMap<String, TokenType>> {
        self.token_maps.get(language)
    }

    /// Get keyword set for a language
    pub fn get_keyword_set(&self, language: &str) -> Option<&KeyWordSet> {
        self.config.keywords.get(language)
    }

    /// Get available languages
    pub fn get_available_languages(&self) -> Vec<&String> {
        self.config.keywords.keys().collect()
    }

    /// Check if a word is a keyword in any language
    pub fn is_keyword(&self, word: &str) -> bool {
        self.token_maps.values()
            .any(|map| map.contains_key(word))
    }

    /// Get TokenType for a word in a specific language
    pub fn get_token_type(&self, word: &str, language: &str) -> Option<TokenType> {
        self.token_maps.get(language)?.get(word).cloned()
    }

    // Helper methods to map language-specific tokens
    fn get_print_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::Moo,
            "es" => TokenType::Muuu,
            _ => TokenType::Moo, // Default fallback
        }
    }

    fn get_array_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::Coop,
            "es" => TokenType::Granja,
            _ => TokenType::Coop,
        }
    }

    fn get_map_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::BarnMap,
            "es" => TokenType::MapaEstablo,
            _ => TokenType::BarnMap,
        }
    }

    fn get_else_if_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::ElseIf,
            "es" => TokenType::SinoSi,
            _ => TokenType::ElseIf,
        }
    }

    fn get_while_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::While,
            "es" => TokenType::Mientras,
            _ => TokenType::While,
        }
    }

    fn get_for_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::For,
            "es" => TokenType::Para,
            _ => TokenType::For,
        }
    }

    fn get_in_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::In,
            "es" => TokenType::En,
            _ => TokenType::In,
        }
    }

    fn get_function_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::Farmfunction,
            "es" => TokenType::Funciongranja,
            _ => TokenType::Farmfunction,
        }
    }

    fn get_return_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::Return,
            "es" => TokenType::Regresa,
            _ => TokenType::Return,
        }
    }

    fn get_is_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::Is,
            "es" => TokenType::Es,
            _ => TokenType::Is,
        }
    }

    fn get_is_not_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::IsNot,
            "es" => TokenType::NoEs,
            _ => TokenType::IsNot,
        }
    }

    fn get_range_token(&self, lang: &str) -> TokenType {
        match lang {
            "en" => TokenType::Barn,
            "es" => TokenType::Granero,
            _ => TokenType::Barn,
        }
    }
}

// ================================
// Configuration Loading & Merging
// ================================

impl Display for MooConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MooConfig {{ version: {}, default_language: {} }}",
               self.language.version, self.language.default_language)
    }
}

impl MooConfig {
    /// Smart config loader with defaults
    pub fn load_with_defaults(path: &Path) -> Result<Self, ConfigError> {
        match Self::load_partial_config(path) {
            Ok(partial_config) => {
                println!("📝 Config loaded, merging with defaults...");
                Ok(Self::merge_with_defaults(partial_config))
            }
            Err(ConfigError::FileNotFoundError(_)) => {
                println!("⚠️ Config file not found, using defaults");
                Ok(Self::from_default())
            }
            Err(err) => Err(err),
        }
    }

    /// Load partial config allowing missing fields
    fn load_partial_config(path: &Path) -> Result<PartialMooConfig, ConfigError> {
        let toml_str = fs::read_to_string(path)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    ConfigError::FileNotFoundError(path.to_string_lossy().to_string())
                },
                _ => ConfigError::IOError(e),
            })?;

        let partial_config: PartialMooConfig = toml::from_str(&toml_str)
            .map_err(|err| ConfigError::ParseError(err.to_string()))?;

        Ok(partial_config)
    }

    /// Merge partial config with defaults
    fn merge_with_defaults(partial: PartialMooConfig) -> Self {
        let language = partial.language.unwrap_or_else(|| LanguageSettings {
            version: "1.0.0".to_string(),
            default_language: "en".to_string(),
        });

        let mut keywords = HashMap::new();
        let defaults = Self::from_default();

        if let Some(partial_keywords) = partial.keywords {
            for (lang, keyword_set) in partial_keywords {
                let default_keywords = defaults.keywords.get(&lang)
                    .cloned()
                    .unwrap_or_else(|| Self::get_generic_defaults_for_language(&lang));

                let merged = Self::merge_keyword_sets(default_keywords, keyword_set);
                keywords.insert(lang, merged);
            }
        }

        if keywords.is_empty() {
            keywords = defaults.keywords;
        }

        Self { language, keywords }
    }

    /// Merge keyword sets
    fn merge_keyword_sets(default: KeyWordSet, partial: PartialKeyWordSet) -> KeyWordSet {
        KeyWordSet {
            print: partial.print.unwrap_or(default.print),
            text_type: partial.text_type.unwrap_or(default.text_type),
            number_type: partial.number_type.unwrap_or(default.number_type),
            decimal_type: partial.decimal_type.unwrap_or(default.decimal_type),
            boolean_type: partial.boolean_type.unwrap_or(default.boolean_type),
            array_type: partial.array_type.unwrap_or(default.array_type),
            map_type: partial.map_type.unwrap_or(default.map_type),
            if_keyword: partial.if_keyword.unwrap_or(default.if_keyword),
            else_if_keyword: partial.else_if_keyword.unwrap_or(default.else_if_keyword),
            else_keyword: partial.else_keyword.unwrap_or(default.else_keyword),
            while_keyword: partial.while_keyword.unwrap_or(default.while_keyword),
            for_keyword: partial.for_keyword.unwrap_or(default.for_keyword),
            in_keyword: partial.in_keyword.unwrap_or(default.in_keyword),
            function_keyword: partial.function_keyword.unwrap_or(default.function_keyword),
            return_keyword: partial.return_keyword.unwrap_or(default.return_keyword),
            is_keyword: partial.is_keyword.unwrap_or(default.is_keyword),
            is_not_keyword: partial.is_not_keyword.unwrap_or(default.is_not_keyword),
            range_keyword: partial.range_keyword.unwrap_or(default.range_keyword),
        }
    }

    /// Get language-specific defaults
    fn get_generic_defaults_for_language(lang: &str) -> KeyWordSet {
        match lang {
            "de" => KeyWordSet {
                print: "muh".to_string(),
                text_type: "text".to_string(),
                number_type: "nummer".to_string(),
                decimal_type: "dezimal".to_string(),
                boolean_type: "boolesch".to_string(),
                array_type: "stall".to_string(),
                map_type: "stall_karte".to_string(),
                if_keyword: "wenn".to_string(),
                else_if_keyword: "sonst wenn".to_string(),
                else_keyword: "sonst".to_string(),
                while_keyword: "während".to_string(),
                for_keyword: "für".to_string(),
                in_keyword: "in".to_string(),
                function_keyword: "bauernfunktion".to_string(),
                return_keyword: "zurück".to_string(),
                is_keyword: "ist".to_string(),
                is_not_keyword: "ist nicht".to_string(),
                range_keyword: "scheune".to_string(),
            },
            "ru" => KeyWordSet {
                print: "му".to_string(),
                text_type: "текст".to_string(),
                number_type: "число".to_string(),
                decimal_type: "дробь".to_string(),
                boolean_type: "логический".to_string(),
                array_type: "загон".to_string(),
                map_type: "карта_сарая".to_string(),
                if_keyword: "если".to_string(),
                else_if_keyword: "иначе если".to_string(),
                else_keyword: "иначе".to_string(),
                while_keyword: "пока".to_string(),
                for_keyword: "для".to_string(),
                in_keyword: "в".to_string(),
                function_keyword: "фермафункция".to_string(),
                return_keyword: "вернуть".to_string(),
                is_keyword: "есть".to_string(),
                is_not_keyword: "не есть".to_string(),
                range_keyword: "сарай".to_string(),
            },
            _ => KeyWordSet {
                print: "print".to_string(),
                text_type: "string".to_string(),
                number_type: "int".to_string(),
                decimal_type: "float".to_string(),
                boolean_type: "bool".to_string(),
                array_type: "array".to_string(),
                map_type: "map".to_string(),
                if_keyword: "if".to_string(),
                else_if_keyword: "elif".to_string(),
                else_keyword: "else".to_string(),
                while_keyword: "while".to_string(),
                for_keyword: "for".to_string(),
                in_keyword: "in".to_string(),
                function_keyword: "function".to_string(),
                return_keyword: "return".to_string(),
                is_keyword: "==".to_string(),
                is_not_keyword: "!=".to_string(),
                range_keyword: "range".to_string(),
            }
        }
    }

    /// Convenience method for simple loading
    pub fn load_smart(path: &Path) -> Self {
        match Self::load_with_defaults(path) {
            Ok(config) => {
                println!("✅ Config loaded successfully!");
                config
            }
            Err(err) => {
                println!("⚠️ Config error: {}, using defaults", err);
                Self::from_default()
            }
        }
    }

    /// Strict TOML loading (your original method)
    pub fn from_toml(path: &Path) -> Result<Self, ConfigError> {
        let toml_str = fs::read_to_string(path)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    ConfigError::FileNotFoundError(path.to_string_lossy().to_string())
                },
                _ => ConfigError::IOError(e),
            })?;

        let config: MooConfig = toml::from_str(&toml_str)
            .map_err(|err| ConfigError::ParseError(err.to_string()))?;

        Ok(config)
    }

    /// Default configuration
    pub fn from_default() -> Self {
        let mut keywords = HashMap::new();

        // English defaults
        keywords.insert("en".to_string(), KeyWordSet {
            print: "moo".to_string(),
            text_type: "text".to_string(),
            number_type: "num".to_string(),
            decimal_type: "dec".to_string(),
            boolean_type: "boolean".to_string(),
            array_type: "coop".to_string(),
            map_type: "barn_map".to_string(),
            if_keyword: "if".to_string(),
            else_if_keyword: "else if".to_string(),
            else_keyword: "else".to_string(),
            while_keyword: "while".to_string(),
            for_keyword: "for".to_string(),
            in_keyword: "in".to_string(),
            function_keyword: "farmfunction".to_string(),
            return_keyword: "return".to_string(),
            is_keyword: "is".to_string(),
            is_not_keyword: "is not".to_string(),
            range_keyword: "barn".to_string(),
        });

        // Spanish defaults
        keywords.insert("es".to_string(), KeyWordSet {
            print: "muuu".to_string(),
            text_type: "texto".to_string(),
            number_type: "numero".to_string(),
            decimal_type: "decimal".to_string(),
            boolean_type: "booleano".to_string(),
            array_type: "granja".to_string(),
            map_type: "mapa_establo".to_string(),
            if_keyword: "si".to_string(),
            else_if_keyword: "sino si".to_string(),
            else_keyword: "sino".to_string(),
            while_keyword: "mientras".to_string(),
            for_keyword: "para".to_string(),
            in_keyword: "en".to_string(),
            function_keyword: "funciongranja".to_string(),
            return_keyword: "regresa".to_string(),
            is_keyword: "es".to_string(),
            is_not_keyword: "no es".to_string(),
            range_keyword: "granero".to_string(),
        });

        Self {
            language: LanguageSettings {
                version: "1.0.0".to_string(),
                default_language: "en".to_string(),
            },
            keywords,
        }
    }

    // Utility methods
    pub fn get_keyword_set(&self, language: &str) -> Option<&KeyWordSet> {
        self.keywords.get(language)
    }

    pub fn get_available_languages(&self) -> Vec<&String> {
        self.keywords.keys().collect()
    }

    pub fn keyword_count(&self, language: &str) -> usize {
        if self.keywords.contains_key(language) { 17 } else { 0 }
    }

    pub fn total_keywords(&self) -> usize {
        self.keywords.len() * 17
    }
}

// ================================
// Helper Structs
// ================================

#[derive(serde::Deserialize, Debug)]
struct PartialMooConfig {
    language: Option<LanguageSettings>,
    keywords: Option<HashMap<String, PartialKeyWordSet>>,
}

#[derive(serde::Deserialize, Debug)]
struct PartialKeyWordSet {
    print: Option<String>,
    text_type: Option<String>,
    number_type: Option<String>,
    decimal_type: Option<String>,
    boolean_type: Option<String>,
    array_type: Option<String>,
    map_type: Option<String>,
    if_keyword: Option<String>,
    else_if_keyword: Option<String>,
    else_keyword: Option<String>,
    while_keyword: Option<String>,
    for_keyword: Option<String>,
    in_keyword: Option<String>,
    function_keyword: Option<String>,
    return_keyword: Option<String>,
    is_keyword: Option<String>,
    is_not_keyword: Option<String>,
    range_keyword: Option<String>,
}

// ================================
// Tests
// ================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config = MooConfig::from_default();
        assert!(config.keywords.contains_key("en"));
        assert!(config.keywords.contains_key("es"));
    }

    #[test]
    fn test_keyword_manager() {
        let config = MooConfig::from_default();
        let manager = LanguageKeywordManager::from_config(config);

        assert!(manager.is_keyword("moo"));
        assert!(manager.is_keyword("muuu"));
        assert_eq!(manager.get_token_type("moo", "en"), Some(TokenType::Moo));
        assert_eq!(manager.get_token_type("muuu", "es"), Some(TokenType::Muuu));
    }
}