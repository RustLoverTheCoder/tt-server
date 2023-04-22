pub struct ApiLanguage {
    pub official: Option<bool>,
    pub rtl: Option<bool>,
    pub beta: Option<bool>,
    pub name: String,
    pub native_name: String,
    pub lang_code: String,
    pub base_lang_code: Option<String>,
    pub plural_code: String,
    pub strings_count: i32,
    pub translated_count: i32,
    pub translations_url: String,
}

pub struct ApiLangString {
    pub key: String,
    pub value: Option<String>,
    pub zero_value: Option<String>,
    pub one_value: Option<String>,
    pub two_value: Option<String>,
    pub few_value: Option<String>,
    pub many_value: Option<String>,
    pub other_value: Option<String>,
}

pub type ApiLangPack = std::collections::HashMap<String, ApiLangString>;