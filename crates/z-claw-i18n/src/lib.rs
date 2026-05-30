use serde::Deserialize;
use std::collections::HashMap;

const EN: &str = include_str!("../locales/en.toml");
const ZH_CN: &str = include_str!("../locales/zh-CN.toml");

#[derive(Debug, Deserialize)]
struct LocaleData {
    entries: HashMap<String, String>,
}

pub struct I18n {
    locale: String,
    active: LocaleData,
    fallback: LocaleData,
}

impl I18n {
    pub fn new(locale: impl Into<String>) -> Self {
        let locale = locale.into();
        let fallback: LocaleData = toml::from_str(EN).expect("failed to parse en.toml");
        let active = match locale.as_str() {
            "zh-CN" => toml::from_str(ZH_CN).expect("failed to parse zh-CN.toml"),
            _ => toml::from_str(EN).expect("failed to parse en.toml"),
        };
        Self {
            locale,
            active,
            fallback,
        }
    }

    pub fn current_locale(&self) -> &str {
        &self.locale
    }

    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.active
            .entries
            .get(key)
            .or_else(|| self.fallback.entries.get(key))
            .map(|s| s.as_str())
            .unwrap_or(key)
    }

    pub fn set_locale(&mut self, locale: impl Into<String>) {
        let locale = locale.into();
        self.active = match locale.as_str() {
            "zh-CN" => toml::from_str(ZH_CN).expect("failed to parse zh-CN.toml"),
            _ => toml::from_str(EN).expect("failed to parse en.toml"),
        };
        self.locale = locale;
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new("en")
    }
}
