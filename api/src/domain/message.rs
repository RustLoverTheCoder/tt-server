use std::collections::HashMap;

use super::{
    bot::ApiWebDocument,
    call::{ApiGroupCall, PhoneCallAction},
    chat::ApiChat,
};

pub struct ApiDimensions {
    width: i32,
    height: i32,
}

pub enum ApiPhotoSizeType {
    S,
    M,
    X,
    Y,
    Z,
}

pub enum ApiVideoSizeType {
    U,
    V,
}

pub struct ApiPhotoSize {
    dimensions: ApiDimensions,
    size_type: ApiPhotoSizeType,
}

pub struct ApiVideoSize {
    dimensions: ApiDimensions,
    size_type: ApiVideoSizeType,
    video_start_ts: i64,
    size: i32,
}

pub struct ApiThumbnail {
    width: i32,
    height: i32,
    data_uri: String,
}

pub struct ApiPhoto {
    id: String,
    thumbnail: Option<ApiThumbnail>,
    is_video: Option<bool>,
    sizes: Vec<ApiPhotoSize>,
    video_sizes: Option<Vec<ApiVideoSize>>,
    blob_url: Option<String>,
    is_spoiler: Option<bool>,
}

pub enum ApiStickerSetInfo {
    ShortName(String),
    Id(String, String),
    Missing,
}

pub struct ApiSticker {
    id: String,
    sticker_set_info: ApiStickerSetInfo,
    emoji: Option<String>,
    is_custom_emoji: bool,
    is_lottie: bool,
    is_video: bool,
    width: Option<i32>,
    height: Option<i32>,
    thumbnail: Option<ApiThumbnail>,
    is_preloaded_globally: Option<bool>,
    has_effect: Option<bool>,
    is_free: Option<bool>,
    should_use_text_color: Option<bool>,
}

pub struct ApiStickerSet {
    is_archived: Option<bool>,
    is_lottie: Option<bool>,
    is_videos: Option<bool>,
    is_emoji: Option<bool>,
    installed_date: Option<i64>,
    id: String,
    access_hash: String,
    title: String,
    has_thumbnail: Option<bool>,
    count: i32,
    stickers: Option<Vec<ApiSticker>>,
    packs: Option<HashMap<String, Vec<ApiSticker>>>,
    covers: Option<Vec<ApiSticker>>,
    short_name: String,
}

pub struct ApiVideo {
    id: String,
    mime_type: String,
    duration: i32,
    file_name: String,
    width: Option<i32>,
    height: Option<i32>,
    supports_streaming: Option<bool>,
    is_round: Option<bool>,
    is_gif: Option<bool>,
    is_spoiler: Option<bool>,
    thumbnail: Option<ApiThumbnail>,
    blob_url: Option<String>,
    preview_blob_url: Option<String>,
    size: i32,
}

pub struct ApiAudio {
    id: String,
    size: i32,
    mime_type: String,
    file_name: String,
    duration: i32,
    performer: Option<String>,
    title: Option<String>,
    thumbnail_sizes: Option<Vec<ApiPhotoSize>>,
}

pub struct ApiVoice {
    id: String,
    duration: i32,
    waveform: Option<Vec<i32>>,
}

pub struct ApiDocument {
    id: Option<String>,
    file_name: String,
    size: i32,
    timestamp: Option<i64>,
    mime_type: String,
    thumbnail: Option<ApiThumbnail>,
    preview_blob_url: Option<String>,
    media_type: Option<String>,
    media_size: Option<ApiDimensions>,
}

pub struct ApiContact {
    first_name: String,
    last_name: String,
    phone_number: String,
    user_id: String,
}

pub struct ApiPollAnswer {
    text: String,
    option: String,
}

pub struct ApiPollResult {
    is_chosen: Option<bool>,
    is_correct: Option<bool>,
    option: String,
    voters_count: i32,
}

pub struct ApiPoll {
    id: String,
    summary: ApiPollSummary,
    results: ApiPollResults,
}

pub struct ApiPollSummary {
    closed: Option<bool>,
    is_public: Option<bool>,
    multiple_choice: Option<bool>,
    quiz: Option<bool>,
    question: String,
    answers: Vec<ApiPollAnswer>,
    close_period: Option<i32>,
    close_date: Option<i64>,
}

pub struct ApiPollResults {
    is_min: Option<bool>,
    results: Option<Vec<ApiPollResult>>,
    total_voters: Option<i32>,
    recent_voter_ids: Option<Vec<String>>,
    solution: Option<String>,
    solution_entities: Option<Vec<ApiMessageEntity>>,
}

pub enum ApiMessageEntity {
    Default {
        offset: i32,
        length: i32,
        entity_type: String,
    },
    Pre {
        offset: i32,
        length: i32,
        language: Option<String>,
    },
    TextUrl {
        offset: i32,
        length: i32,
        url: String,
    },
    MentionName {
        offset: i32,
        length: i32,
        user_id: String,
    },
    CustomEmoji {
        offset: i32,
        length: i32,
        document_id: String,
    },
}

pub enum ApiInputInvoice {
    ChatIdMessageId {
        chat_id: String,
        message_id: i32,
        is_extended_media: Option<bool>,
    },
    Slug(String),
}

pub enum ApiRequestInputInvoice {
    ChatMessage { chat: ApiChat, message_id: i32 },
    Slug(String),
}

pub struct ApiInvoice {
    text: String,
    title: String,
    photo: Option<ApiWebDocument>,
    amount: i32,
    currency: String,
    receipt_msg_id: Option<i32>,
    is_test: Option<bool>,
    is_recurring: Option<bool>,
    recurring_terms_url: Option<String>,
    extended_media: Option<ApiMessageExtendedMediaPreview>,
    max_tip_amount: Option<i32>,
    suggested_tip_amounts: Option<Vec<i32>>,
}

pub struct ApiMessageExtendedMediaPreview {
    width: Option<i32>,
    height: Option<i32>,
    thumbnail: Option<ApiThumbnail>,
    duration: Option<i32>,
}

pub struct ApiPaymentCredentials {
    id: String,
    title: String,
}

pub struct ApiGeoPoint {
    long: f64,
    lat: f64,
    access_hash: String,
    accuracy_radius: Option<i32>,
}

pub enum ApiLocation {
    Geo(ApiGeo),
    Venue(ApiVenue),
    GeoLive(ApiGeoLive),
}

pub struct ApiGeo {
    geo: ApiGeoPoint,
}

pub struct ApiVenue {
    geo: ApiGeoPoint,
    title: String,
    address: String,
    provider: String,
    venue_id: String,
    venue_type: String,
}

pub struct ApiGeoLive {
    geo: ApiGeoPoint,
    heading: Option<i32>,
    period: i32,
}

pub struct ApiGame {
    title: String,
    description: String,
    photo: Option<ApiPhoto>,
    short_name: String,
    id: String,
    access_hash: String,
    document: Option<ApiDocument>,
}

pub struct ApiNewPoll {
    summary: ApiPollSummary,
    quiz: Option<ApiQuiz>,
}

pub struct ApiQuiz {
    correct_answers: Vec<String>,
    solution: Option<String>,
    solution_entities: Option<Vec<ApiMessageEntity>>,
}

pub struct ApiAction {
    text: String,
    target_user_ids: Option<Vec<String>>,
    target_chat_id: Option<String>,
    r#type: ApiActionType,
    photo: Option<ApiPhoto>,
    amount: Option<i32>,
    currency: Option<String>,
    translation_values: Vec<String>,
    call: Option<ApiGroupCall>,
    phone_call: Option<PhoneCallAction>,
    score: Option<i32>,
    months: Option<i32>,
    topic_emoji_icon_id: Option<String>,
    is_topic_action: Option<bool>,
}

pub enum ApiActionType {
    HistoryClear,
    ContactSignUp,
    ChatCreate,
    TopicCreate,
    SuggestProfilePhoto,
    Other,
}

pub struct ApiWebPage {
    id: i32,
    url: String,
    display_url: String,
    r#type: Option<String>,
    site_name: Option<String>,
    title: Option<String>,
    description: Option<String>,
    photo: Option<ApiPhoto>,
    duration: Option<i32>,
    document: Option<ApiDocument>,
}

pub struct ApiMessageForwardInfo {
    pub date: i64,
    pub is_imported: Option<bool>,
    pub is_channel_post: bool,
    pub channel_post_id: Option<i64>,
    pub is_linked_channel_post: Option<bool>,
    pub from_chat_id: Option<String>,
    pub sender_user_id: Option<String>,
    pub from_message_id: Option<i64>,
    pub hidden_user_name: Option<String>,
    pub post_author_title: Option<String>,
}

#[derive(Debug)]
pub enum ApiMessageEntityTypes {
    Bold,
    Blockquote,
    BotCommand,
    Cashtag,
    Code,
    Email,
    Hashtag,
    Italic,
    MentionName,
    Mention,
    Phone,
    Pre,
    Strike,
    TextUrl,
    Url,
    Underline,
    Spoiler,
    CustomEmoji,
    Unknown,
}

pub struct ApiFormattedText {
    text: String,
    entities: Option<Vec<ApiMessageEntity>>,
}

pub struct ApiMessage {
    id: i32,
    chat_id: String,
    content: ApiMessageContent,
    date: i64,
    is_outgoing: bool,
    sender_id: Option<String>,
    reply_to_chat_id: Option<String>,
    reply_to_message_id: Option<i32>,
    reply_to_top_message_id: Option<i32>,
    is_topic_reply: Option<bool>,
    sending_state: Option<String>,
    forward_info: Option<ApiMessageForwardInfo>,
    is_deleting: Option<bool>,
    previous_local_id: Option<i32>,
    views: Option<i32>,
    forwards: Option<i32>,
    is_edited: Option<bool>,
    edit_date: Option<i64>,
    is_mentioned: Option<bool>,
    is_media_unread: Option<bool>,
    grouped_id: Option<String>,
    is_in_album: Option<bool>,
    has_unread_mention: Option<bool>,
    inline_buttons: Option<ApiKeyboardButtons>,
    keyboard_buttons: Option<ApiKeyboardButtons>,
    keyboard_placeholder: Option<String>,
    is_keyboard_single_use: Option<bool>,
    is_keyboard_selective: Option<bool>,
    via_bot_id: Option<String>,
    replies_thread_info: Option<ApiThreadInfo>,
    post_author_title: Option<String>,
    is_scheduled: Option<bool>,
    should_hide_keyboard_buttons: Option<bool>,
    is_hide_keyboard_selective: Option<bool>,
    is_from_scheduled: Option<bool>,
    is_silent: Option<bool>,
    is_pinned: Option<bool>,
    seen_by_user_ids: Option<Vec<String>>,
    is_protected: Option<bool>,
    is_forwarding_allowed: Option<bool>,
    transcription_id: Option<String>,
    is_transcription_error: Option<bool>,
    emoji_only_count: Option<i32>,
    reactors: Option<ApiMessageReactors>,
    reactions: Option<ApiReactions>,
}

pub struct ApiMessageContent {
    text: Option<ApiFormattedText>,
    photo: Option<ApiPhoto>,
    video: Option<ApiVideo>,
    document: Option<ApiDocument>,
    sticker: Option<ApiSticker>,
    contact: Option<ApiContact>,
    poll: Option<ApiPoll>,
    action: Option<ApiAction>,
    web_page: Option<ApiWebPage>,
    audio: Option<ApiAudio>,
    voice: Option<ApiVoice>,
    invoice: Option<ApiInvoice>,
    location: Option<ApiLocation>,
    game: Option<ApiGame>,
}

pub struct ApiMessageReactors {
    next_offset: Option<String>,
    count: i32,
    reactions: Vec<ApiUserReaction>,
}

pub struct ApiReactions {
    can_see_list: Option<bool>,
    results: Vec<ApiReactionCount>,
    recent_reactions: Option<Vec<ApiUserReaction>>,
}

pub struct ApiUserReaction {
    user_id: String,
    reaction: ApiReaction,
    is_big: Option<bool>,
    is_unread: Option<bool>,
}

pub struct ApiReactionCount {
    chosen_order: Option<i32>,
    count: i32,
    reaction: ApiReaction,
}

pub enum ApiReaction {
    Emoji { emoticon: String },
    CustomEmoji { document_id: String },
}

pub struct ApiAvailableReaction {
    select_animation: Option<ApiDocument>,
    activate_animation: Option<ApiDocument>,
    effect_animation: Option<ApiDocument>,
    static_icon: Option<ApiDocument>,
    center_icon: Option<ApiDocument>,
    around_animation: Option<ApiDocument>,
    reaction: ApiReactionEmoji,
    title: String,
    is_inactive: Option<bool>,
    is_premium: Option<bool>,
}

pub struct ApiChatReactionsAll {
    type_: String,
    are_custom_allowed: Option<bool>,
}

pub struct ApiChatReactionsSome {
    type_: String,
    allowed: Vec<ApiReaction>,
}

pub enum ApiChatReactions {
    All(ApiChatReactionsAll),
    Some(ApiChatReactionsSome),
}

pub struct ApiReactionEmoji {
    emoticon: String,
}

pub struct ApiReactionCustomEmoji {
    document_id: String,
}

pub struct ApiThreadInfo {
    pub is_comments: Option<bool>,
    pub thread_id: i64,
    pub chat_id: String,
    pub top_message_id: Option<i64>,
    pub origin_channel_id: Option<String>,
    pub messages_count: i64,
    pub last_message_id: Option<i64>,
    pub last_read_inbox_message_id: Option<i64>,
    pub recent_replier_ids: Option<Vec<String>>,
}

pub enum ApiMessageOutgoingStatus {
    Read,
    Succeeded,
    Pending,
    Failed,
}

pub struct ApiSponsoredMessage {
    chat_id: Option<String>,
    random_id: String,
    is_recommended: Option<bool>,
    is_bot: Option<bool>,
    channel_post_id: Option<i32>,
    start_param: Option<String>,
    chat_invite_hash: Option<String>,
    chat_invite_title: Option<String>,
    text: ApiFormattedText,
    expires_at: i64,
}

// KeyboardButtons

pub struct ApiKeyboardButtonSimple {
    type_: String, // use "type_" instead of "type" since "type" is a reserved keyword in Rust
    text: String,
}

pub struct ApiKeyboardButtonReceipt {
    type_: String,
    text: String,
    receipt_message_id: i32, // use snake_case instead of camelCase for consistency with Rust naming conventions
}

pub struct ApiKeyboardButtonUrl {
    type_: String,
    text: String,
    url: String,
}

pub struct ApiKeyboardButtonSimpleWebView {
    type_: String,
    text: String,
    url: String,
}

pub struct ApiKeyboardButtonWebView {
    type_: String,
    text: String,
    url: String,
}

pub struct ApiKeyboardButtonCallback {
    type_: String,
    text: String,
    data: String,
}

pub struct ApiKeyboardButtonRequestPoll {
    type_: String,
    text: String,
    is_quiz: Option<bool>,
}

pub struct ApiKeyboardButtonSwitchBotInline {
    type_: String,
    text: String,
    query: String,
    is_same_peer: Option<bool>,
}

pub struct ApiKeyboardButtonUserProfile {
    type_: String,
    text: String,
    user_id: String,
}

pub struct ApiKeyboardButtonUrlAuth {
    pub type_: String,
    pub text: String,
    pub url: String,
    pub button_id: i32,
}

pub struct ApiTranscription {
    pub text: String,
    pub is_pending: Option<bool>,
    pub transcription_id: String,
}

pub enum ApiKeyboardButton {
    Simple(ApiKeyboardButtonSimple),
    Receipt(ApiKeyboardButtonReceipt),
    Url(ApiKeyboardButtonUrl),
    Callback(ApiKeyboardButtonCallback),
    RequestPoll(ApiKeyboardButtonRequestPoll),
    SwitchBotInline(ApiKeyboardButtonSwitchBotInline),
    UserProfile(ApiKeyboardButtonUserProfile),
    WebView(ApiKeyboardButtonWebView),
    SimpleWebView(ApiKeyboardButtonSimpleWebView),
    UrlAuth(ApiKeyboardButtonUrlAuth),
}

type ApiKeyboardButtons = Vec<Vec<ApiKeyboardButton>>;

struct ApiReplyKeyboard {
    keyboard_placeholder: Option<String>,
    is_keyboard_single_use: Option<bool>,
    is_keyboard_selective: Option<bool>,
    inline_buttons: Option<ApiKeyboardButtons>,
    keyboard_buttons: Option<ApiKeyboardButtons>,
}

pub enum ApiMessageSearchType {
    Text,
    Media,
    Documents,
    Links,
    Audio,
    Voice,
    ProfilePhoto,
}

pub enum ApiGlobalMessageSearchType {
    Text,
    Media,
    Documents,
    Links,
    Audio,
    Voice,
}

pub enum ApiReportReason {
    Spam,
    Violence,
    Pornography,
    ChildAbuse,
    Copyright,
    GeoIrrelevant,
    Fake,
    IllegalDrugs,
    PersonalDetails,
    Other,
}

pub enum ApiSendMessageAction {
    Cancel,
    Typing,
    RecordAudio,
    ChooseSticker,
    PlayingGame,
}

pub struct ApiThemeParameters {
    bg_color: String,
    text_color: String,
    hint_color: String,
    link_color: String,
    button_color: String,
    button_text_color: String,
    secondary_bg_color: String,
}

pub const MAIN_THREAD_ID: i32 = -1;

pub const MESSAGE_DELETED: &str = "MESSAGE_DELETED";
