use super::{message::{ApiDocument, ApiPhoto, ApiReaction}, user::ApiUser};

pub struct ApiInitialArgs {
    user_agent: String,
    platform: Option<String>,
    session_data: Option<ApiSessionData>,
    is_test: Option<bool>,
    is_mov_supported: Option<bool>,
    is_webm_supported: Option<bool>,
    max_buffer_size: Option<usize>,
    web_auth_token: Option<String>,
    dc_id: Option<u32>,
    mock_scenario: Option<String>,
}

// ApiOnProgress trait
pub trait ApiOnProgress {
    fn on_progress(&self, progress: f32, args: &[&dyn std::any::Any]);
    fn is_canceled(&self) -> bool { false }
    fn accepts_buffer(&self) -> bool { false }
}

impl<F> ApiOnProgress for F
where
    F: Fn(f32, &[&dyn std::any::Any]),
{
    fn on_progress(&self, progress: f32, args: &[&dyn std::any::Any]) {
        self(progress, args);
    }
}


pub struct ApiAttachment {
    blob_url: String,
    compressed_blob_url: Option<String>,
    filename: String,
    mime_type: String,
    size: u64,
    quick: Option<Quick>,
    voice: Option<Voice>,
    audio: Option<Audio>,
    preview_blob_url: Option<String>,
    should_send_as_file: Option<bool>,
    should_send_as_spoiler: Option<bool>,
    unique_id: Option<String>,
}

struct Quick {
    width: u32,
    height: u32,
    duration: Option<u32>,
}

struct Voice {
    duration: u32,
    waveform: Option<Vec<u8>>,
}

struct Audio {
    duration: u32,
    title: Option<String>,
    performer: Option<String>,
}


// ApiWallpaper 结构体
pub struct ApiWallpaper {
    slug: String,
    document: ApiDocument,
}

// ApiSession 结构体
pub struct ApiSession {
    hash: String,
    is_current: bool,
    is_official_app: bool,
    is_password_pending: bool,
    device_model: String,
    platform: String,
    system_version: String,
    app_name: String,
    app_version: String,
    date_created: i64,
    date_active: i64,
    ip: String,
    country: String,
    region: String,
    are_calls_enabled: bool,
    are_secret_chats_enabled: bool,
}

struct ApiWebSession {
    hash: String,
    bot_id: String,
    domain: String,
    browser: String,
    platform: String,
    date_created: i64,
    date_active: i64,
    ip: String,
    region: String,
}

// ApiSessionData 结构体
struct ApiSessionData {
    main_dc_id: u32,
    keys: std::collections::HashMap<u32, KeyType>,
    hashes: std::collections::HashMap<u32, HashType>,
}

enum KeyType {
    String(String),
    Number(Vec<u32>),
}

enum HashType {
    String(String),
    Number(Vec<u32>),
}

// ApiNotifyException 结构体
struct ApiNotifyException {
    chat_id: String,
    is_muted: bool,
    is_silent: Option<bool>,
    should_show_previews: Option<bool>,
}

// ApiNotification 结构体
struct ApiNotification {
    local_id: String,
    title: Option<String>,
    message: String,
    action_text: Option<String>,
    action: Option<CallbackAction>,
    class_name: Option<String>,
}

// CallbackAction 枚举类型
enum CallbackAction {
    Function(Box<dyn Fn() -> ()>),
    Url(String),
}

// ApiError 结构体
struct ApiError {
    message: String,
    has_error_key: Option<bool>,
    is_slow_mode: Option<bool>,
    text_params: Option<std::collections::HashMap<String, String>>,
}

// ApiFieldError 结构体
struct ApiFieldError {
    field: String,
    message: String,
}

struct ApiInviteInfo {
    title: String,
    about: Option<String>,
    hash: String,
    is_channel: Option<bool>,
    participants_count: Option<u32>,
    is_request_needed: Option<bool>,
    photo: Option<ApiPhoto>,
}

// ApiExportedInvite 结构体
struct ApiExportedInvite {
    is_revoked: Option<bool>,
    is_permanent: Option<bool>,
    link: String,
    date: i64,
    start_date: Option<i64>,
    expire_date: Option<i64>,
    usage_limit: Option<u32>,
    usage: Option<u32>,
    is_request_needed: Option<bool>,
    requested: Option<i64>,
    title: Option<String>,
    admin_id: String,
}


// ApiChatInviteImporter 结构体
struct ApiChatInviteImporter {
    user_id: String,
    date: i64,
    is_requested: Option<bool>,
    about: Option<String>,
}

// ApiCountry 结构体
struct ApiCountry {
    is_hidden: Option<bool>,
    iso2: String,
    default_name: String,
    name: Option<String>,
}

// ApiCountryCode 结构体
struct ApiCountryCode {
    country_code: String,
    prefixes: Option<Vec<String>>,
    patterns: Option<Vec<String>>,
    is_hidden: Option<bool>,
    iso2: String,
    default_name: String,
    name: Option<String>,
}

// ApiAppConfig 结构体
struct ApiAppConfig {
    emoji_sounds: std::collections::HashMap<String, String>,
    seen_by_max_chat_members: i32,
    seen_by_expires_at: i64,
    autologin_domains: Vec<String>,
    autologin_token: String,
    url_auth_domains: Vec<String>,
    premium_invoice_slug: String,
    premium_bot_username: String,
    is_premium_purchase_blocked: bool,
    premium_promo_order: Vec<String>,
    default_emoji_statuses_sticker_set_id: String,
    max_unique_reactions: i32,
    topics_pinned_limit: i32,
    max_user_reactions_default: i32,
    max_user_reactions_premium: i32,
    hidden_members_min_count: i32,
    limits: std::collections::HashMap<ApiLimitType, (i32, i32)>,
    can_display_autoarchive_setting: bool,
}

// ApiConfig 结构体
struct ApiConfig {
    expires_at: i64,
    default_reaction: Option<ApiReaction>,
    gif_search_username: Option<String>,
    max_group_size: i32,
}

struct GramJsEmojiInteraction {
    v: i32,
    a: Vec<GramJsEmojiInteractionAction>,
}

// GramJsEmojiInteractionAction 结构体
struct GramJsEmojiInteractionAction {
    i: i32,
    t: i32,
}

// ApiEmojiInteraction 结构体
struct ApiEmojiInteraction {
    timestamps: Vec<i64>,
}

// ApiUrlAuthResultRequest 结构体
struct ApiUrlAuthResultRequest {
    type_: String,
    bot: ApiUser,
    domain: String,
    should_request_write_access: Option<bool>,
}

// ApiUrlAuthResultAccepted 结构体
struct ApiUrlAuthResultAccepted {
    type_: String,
    url: String,
}

// ApiUrlAuthResultDefault 结构体
struct ApiUrlAuthResultDefault {
    type_: String,
}

// ApiUrlAuthResult 枚举类型
enum ApiUrlAuthResult {
    Request(ApiUrlAuthResultRequest),
    Accepted(ApiUrlAuthResultAccepted),
    Default(ApiUrlAuthResultDefault),
}

// ApiLimitType 枚举类型
enum ApiLimitType {
    UploadMaxFileparts,
    StickersFaved,
    SavedGifs,
    DialogFiltersChats,
    DialogFilters,
    DialogFolderPinned,
    CaptionLength,
    Channels,
    ChannelsPublic,
    AboutLength,
}