use super::{message::{ApiPhoto, ApiDocument}, bot::ApiBotInfo};

pub struct ApiUser {
    pub id: String,
    pub is_min: bool,
    pub is_self: Option<bool>,
    pub is_verified: Option<bool>,
    pub is_premium: Option<bool>,
    pub is_contact: Option<bool>,
    pub r#type: ApiUserType,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub no_status: Option<bool>,
    pub usernames: Option<Vec<ApiUsername>>,
    pub phone_number: String,
    pub access_hash: Option<String>,
    pub has_video_avatar: Option<bool>,
    pub avatar_hash: Option<String>,
    pub photos: Option<Vec<ApiPhoto>>,
    pub bot_placeholder: Option<String>,
    pub can_be_invited_to_group: Option<bool>,
    pub common_chats: Option<ApiCommonChats>,
    pub fake_type: Option<ApiFakeType>,
    pub is_attach_bot: Option<bool>,
    pub emoji_status: Option<ApiEmojiStatus>,
    pub full_info: Option<ApiUserFullInfo>,
}

pub struct ApiUserFullInfo {
    pub is_blocked: Option<bool>,
    pub bio: Option<String>,
    pub common_chats_count: Option<i32>,
    pub pinnedMessageId: Option<i32>,
    pub bot_info: Option<ApiBotInfo>,
    pub profilePhoto: Option<ApiPhoto>,
    pub fallback_photo: Option<ApiPhoto>,
    pub personal_photo: Option<ApiPhoto>,
    pub no_voice_messages: Option<bool>,
    pub premium_gifts: Option<Vec<ApiPremiumGiftOption>>,
}

pub type ApiFakeType = String;

pub type ApiUserType = String;

pub struct ApiUserStatus {
    pub r#type: String,
    pub was_online: Option<i32>,
    pub expires: Option<i32>,
}

pub struct ApiUsername {
    pub username: String,
    pub is_active: Option<bool>,
    pub is_editable: Option<bool>,
}

pub type ApiChatType = String;

pub type ApiAttachMenuPeerType = String;

pub struct ApiAttachBot {
    pub id: String,
    pub has_settings: Option<bool>,
    pub should_request_write_access: Option<bool>,
    pub short_name: String,
    pub peer_types: Vec<ApiAttachMenuPeerType>,
    pub icons: Vec<ApiAttachBotIcon>,
}

pub struct ApiAttachBotIcon {
    pub name: String,
    pub document: ApiDocument,
}

pub struct ApiPremiumGiftOption {
    pub months: i32,
    pub currency: String,
    pub amount: i32,
    pub bot_url: String,
}

pub struct ApiEmojiStatus {
    pub document_id: String,
    pub until: Option<i32>,
}

pub struct ApiCommonChats {
    pub ids: Vec<String>,
    pub max_id: String,
    pub is_fully_loaded: bool,
}