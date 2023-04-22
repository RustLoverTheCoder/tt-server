use super::message::{ApiSticker, ApiPhoto, ApiDimensions, ApiVideo, ApiThumbnail};

pub type ApiInlineResultType = &'static str;

pub struct ApiWebDocument {
    pub url: String,
    pub size: i32,
    pub mime_type: String,
    pub access_hash: Option<String>,
    pub dimensions: Option<ApiDimensions>,
}

pub struct ApiBotInlineResult {
    pub id: String,
    pub query_id: String,
    pub r#type: ApiInlineResultType,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub web_thumbnail: Option<ApiWebDocument>,
}

pub struct ApiBotInlineMediaResult {
    pub id: String,
    pub query_id: String,
    pub r#type: ApiInlineResultType,
    pub title: Option<String>,
    pub description: Option<String>,
    pub sticker: Option<ApiSticker>,
    pub photo: Option<ApiPhoto>,
    pub gif: Option<ApiVideo>,
    pub thumbnail: Option<ApiThumbnail>,
}

pub struct ApiBotInlineSwitchPm {
    pub text: String,
    pub start_param: String,
}

pub struct ApiBotCommand {
    pub bot_id: String,
    pub command: String,
    pub description: String,
}

pub enum ApiBotMenuButtonType {
    Commands,
    WebApp { text: String, url: String },
}

pub struct ApiBotMenuButton {
    pub r#type: ApiBotMenuButtonType,
}

pub struct ApiBotInfo {
    pub bot_id: String,
    pub commands: Option<Vec<ApiBotCommand>>,
    pub description: Option<String>,
    pub photo: Option<ApiPhoto>,
    pub gif: Option<ApiVideo>,
    pub menu_button: ApiBotMenuButton,
}