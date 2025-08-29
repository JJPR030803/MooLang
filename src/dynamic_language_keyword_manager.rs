
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;
use crate::tokens::TokenType;
use crate::config::{MooLangConfig, KeywordSet};

pub struct DynamicLanguageKeywords {
    config: MooLangConfig,
    keyword_maps: HashMap<String, HashMap<String, TokenType>>,
}

impl DynamicLanguageKeywords {
    fn new() -> Self {
        // Try to load from config file, fall back to default if not found
        let config = MooLangConfig::from_file("moolang_config.toml")
            .unwrap_or_else(|_| {
                println!("Config file not found, using default configuration");
                MooLangConfig::from_default()
            });
        
        let mut instance = Self {
            config,
            keyword_maps: HashMap::new(),
        };
        
        instance.build_keyword_maps();
        instance
    }
    
    fn build_keyword_maps(&mut self) {
        for (lang, keyword_set) in &self.config.keywords {
            let mut map = HashMap::new();
            
            // Map custom keywords to TokenTypes
            map.insert(keyword_set.print.clone(), TokenType::Moo);
            map.insert(keyword_set.text_type.clone(), TokenType::Text);
            map.insert(keyword_set.number_type.clone(), TokenType::Num);
            map.insert(keyword_set.decimal_type.clone(), TokenType::Dec);
            map.insert(keyword_set.boolean_type.clone(), TokenType::Boolean);
            map.insert(keyword_set.array_type.clone(), TokenType::Coop);
            map.insert(keyword_set.map_type.clone(), TokenType::BarnMap);
            map.insert(keyword_set.if_keyword.clone(), TokenType::If);
            map.insert(keyword_set.else_if_keyword.clone(), TokenType::ElseIf);
            map.insert(keyword_set.else_keyword.clone(), TokenType::Else);
            map.insert(keyword_set.while_keyword.clone(), TokenType::While);
            map.insert(keyword_set.for_keyword.clone(), TokenType::For);
            map.insert(keyword_set.in_keyword.clone(), TokenType::In);
            map.insert(keyword_set.function_keyword.clone(), TokenType::Farmfunction);
            map.insert(keyword_set.return_keyword.clone(), TokenType::Return);
            map.insert(keyword_set.is_keyword.clone(), TokenType::Is);
            map.insert(keyword_set.is_not_keyword.clone(), TokenType::IsNot);
            map.insert(keyword_set.range_keyword.clone(), TokenType::Barn);
            
            self.keyword_maps.insert(lang.clone(), map);
        }
    }
    
    pub fn get_keywords(&self, language: &str) -> Option<&HashMap<String, TokenType>> {
        self.keyword_maps.get(language)
    }
    
    pub fn get_config(&self) -> &MooLangConfig {
        &self.config
    }
    
    pub fn reload_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let new_config = MooLangConfig::from_file("moolang_config.toml")?;
        self.config = new_config;
        self.keyword_maps.clear();
        self.build_keyword_maps();
        println!("Configuration reloaded successfully");
        Ok(())
    }
    
    pub fn get_available_languages(&self) -> Vec<String> {
        self.config.get_available_languages()
    }
}

lazy_static! {
    pub static ref LANGUAGE_KEYWORDS: RwLock<DynamicLanguageKeywords> = 
        RwLock::new(DynamicLanguageKeywords::new());
    
    pub static ref KEYWORD_STATS: HashMap<&'static str, usize> = {
        let mut stats = HashMap::new();
        let keywords = LANGUAGE_KEYWORDS.read().unwrap();
        let config = keywords.get_config();
        
        stats.insert("total_languages", config.get_available_languages().len());
        stats.insert("keywords_per_language", 17);
        stats.insert("total_keywords", config.total_keywords());
        
        stats
    };
}
