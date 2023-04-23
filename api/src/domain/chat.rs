use std::collections::HashMap;

use super::{user::{ApiUsername, ApiFakeType}, message::{ApiMessage, ApiPhoto, ApiStickerSet, ApiChatReactions}, bot::ApiBotCommand, misc::ApiChatInviteImporter};

pub struct ApiChat {
    pub id: String,
    pub folder_id: Option<u32>,
    pub chat_type: ApiChatType,
    pub title: String,
    pub has_unread_mark: Option<bool>,
    pub last_message: Option<ApiMessage>,
    pub last_read_outbox_message_id: Option<u32>,
    pub last_read_inbox_message_id: Option<u32>,
    pub unread_count: Option<u32>,
    pub unread_mentions_count: Option<u32>,
    pub unread_reactions_count: Option<u32>,
    pub is_verified: Option<bool>,
    pub is_muted: Option<bool>,
    pub is_signatures_shown: Option<bool>,
    pub has_private_link: Option<bool>,
    pub access_hash: Option<String>,
    pub is_min: Option<bool>,
    pub has_video_avatar: Option<bool>,
    pub avatar_hash: Option<String>,
    pub usernames: Option<Vec<ApiUsername>>,
    pub members_count: Option<u32>,
    pub join_date: Option<u32>,
    pub is_support: Option<bool>,
    pub photos: Option<Vec<ApiPhoto>>,
    pub draft_date: Option<u32>,
    pub is_protected: Option<bool>,
    pub fake_type: Option<ApiFakeType>,
    pub is_forum: Option<bool>,
    pub topics: Option<HashMap<u32, ApiTopic>>,
    pub listed_topic_ids: Option<Vec<u32>>,
    pub topics_count: Option<u32>,
    pub ordered_pinned_topic_ids: Option<Vec<u32>>,

    // Calls
    pub is_call_active: Option<bool>,
    pub is_call_not_empty: Option<bool>,

    // Current user permissions
    pub is_not_joined: Option<bool>,
    pub is_listed: Option<bool>,
    pub is_creator: Option<bool>,
    pub is_forbidden: Option<bool>, // Forbidden - can't send messages (user was kicked, for example)
    pub is_restricted: Option<bool>, // Restricted - can't access the chat (user was banned or chat is violating rules)
    pub restriction_reason: Option<ApiRestrictionReason>,
    pub admin_rights: Option<ApiChatAdminRights>,
    pub current_user_banned_rights: Option<ApiChatBannedRights>,
    pub default_banned_rights: Option<ApiChatBannedRights>,

    pub migrated_to: Option<MigratedTo>,

    // Obtained from GetChatSettings
    pub settings: Option<ApiChatSettings>,
    // Obtained from GetFullChat / GetFullChannel
    pub full_info: Option<ApiChatFullInfo>,

    pub join_requests: Option<Vec<ApiChatInviteImporter>>,
    pub is_join_to_send: Option<bool>,
    pub is_join_request: Option<bool>,
    pub send_as_peer_ids: Option<Vec<ApiSendAsPeerId>>,

    pub unread_reactions: Option<Vec<u32>>,
    pub unread_mentions: Option<Vec<u32>>,
}

pub enum ApiChatType {
    ChatTypePrivate,
    ChatTypeSecret,
    ChatTypeBasicGroup,
    ChatTypeSuperGroup,
    ChatTypeChannel,
}

pub struct MigratedTo {
    pub chat_id: String,
    pub access_hash: Option<String>,
}

pub struct ApiTypingStatus {
    user_id: Option<String>,
    action: String,
    timestamp: i64,
    emoji: Option<String>,
}

pub struct ApiChatFullInfo {
    about: Option<String>,
    online_count: Option<i32>,
    members: Option<Vec<ApiChatMember>>,
    kicked_members: Option<Vec<ApiChatMember>>,
    admin_members_by_id: Option<HashMap<String, ApiChatMember>>,
    can_view_members: Option<bool>,
    is_pre_history_hidden: Option<bool>,
    invite_link: Option<String>,
    group_call_id: Option<String>,
    slow_mode: Option<ApiSlowMode>,
    migrated_from: Option<ApiMigratedFrom>,
    linked_chat_id: Option<String>,
    bot_commands: Option<Vec<ApiBotCommand>>,
    enabled_reactions: Option<ApiChatReactions>,
    send_as_id: Option<String>,
    can_view_statistics: Option<bool>,
    recent_requester_ids: Option<Vec<String>>,
    requests_pending: Option<i32>,
    statistics_dc_id: Option<i32>,
    sticker_set: Option<ApiStickerSet>,
    profile_photo: Option<ApiPhoto>,
    are_participants_hidden: Option<bool>,
}

pub struct ApiSlowMode {
    seconds: i32,
    next_send_date: Option<i32>,
}

pub struct ApiMigratedFrom {
    chat_id: String,
    max_message_id: Option<i32>,
}

pub struct ApiChatMember {
    user_id: String,
    inviter_id: Option<String>,
    joined_date: Option<i32>,
    kicked_by_user_id: Option<String>,
    promoted_by_user_id: Option<String>,
    banned_rights: Option<ApiChatBannedRights>,
    admin_rights: Option<ApiChatAdminRights>,
    custom_title: Option<String>,
    is_admin: Option<bool>,
    is_owner: Option<bool>,
}

pub struct ApiChatAdminRights {
    change_info: Option<bool>,
    post_messages: Option<bool>,
    edit_messages: Option<bool>,
    delete_messages: Option<bool>,
    ban_users: Option<bool>,
    invite_users: Option<bool>,
    pin_messages: Option<bool>,
    add_admins: Option<bool>,
    anonymous: Option<bool>,
    manage_call: Option<bool>,
    manage_topics: Option<bool>,
}

pub struct ApiChatBannedRights {
    view_messages: Option<bool>,
    send_messages: Option<bool>,
    send_media: Option<bool>,
    send_stickers: Option<bool>,
    send_gifs: Option<bool>,
    send_games: Option<bool>,
    send_inline: Option<bool>,
    embed_links: Option<bool>,
    send_polls: Option<bool>,
    change_info: Option<bool>,
    invite_users: Option<bool>,
    pin_messages: Option<bool>,
    manage_topics: Option<bool>,
    send_photos: Option<bool>,
    send_videos: Option<bool>,
    send_roundvideos: Option<bool>,
    send_audios: Option<bool>,
    send_voices: Option<bool>,
    send_docs: Option<bool>,
    send_plain: Option<bool>,
    until_date: Option<i32>,
}

pub struct ApiRestrictionReason {
    reason: String,
    text: String,
}

pub struct ApiChatFolder {
    id: i32,
    title: String,
    description: Option<String>,
    emoticon: Option<String>,
    contacts: Option<bool>,
    non_contacts: Option<bool>,
    groups: Option<bool>,
    channels: Option<bool>,
    bots: Option<bool>,
    exclude_muted: Option<bool>,
    exclude_read: Option<bool>,
    exclude_archived: Option<bool>,
    pinned_chat_ids: Option<Vec<String>>,
    included_chat_ids: Vec<String>,
    excluded_chat_ids: Vec<String>,
}

pub struct ApiChatSettings {
    is_auto_archived: Option<bool>,
    can_report_spam: Option<bool>,
    can_add_contact: Option<bool>,
    can_block_contact: Option<bool>,
}

pub struct ApiSendAsPeerId {
    id: String,
    is_premium: Option<bool>,
}

pub struct ApiTopic {
    id: i32,
    is_closed: Option<bool>,
    is_pinned: Option<bool>,
    is_hidden: Option<bool>,
    is_owner: Option<bool>,
    is_min: Option<bool>,
    date: i64,
    title: String,
    icon_color: i32,
    icon_emoji_id: Option<String>,
    last_message_id: i32,
    unread_count: i32,
    unread_mentions_count: i32,
    unread_reactions_count: i32,
    from_id: String,
    is_muted: Option<bool>,
}