use std::collections::HashMap;

pub struct ApiChat {
    pub id: String,
    pub folder_id: Option<i32>,
    pub chat_type: ApiChatType,
    pub title: String,
    pub has_unread_mark: Option<bool>,
    pub last_message: Option<ApiMessage>,
    pub last_read_outbox_message_id: Option<i32>,
    pub last_read_inbox_message_id: Option<i32>,
    pub unread_count: Option<i32>,
    pub unread_mentions_count: Option<i32>,
    pub unread_reactions_count: Option<i32>,
    pub is_verified: Option<bool>,
    pub is_muted: Option<bool>,
    pub is_signatures_shown: Option<bool>,
    pub has_private_link: Option<bool>,
    pub access_hash: Option<String>,
    pub is_min: Option<bool>,
    pub has_video_avatar: Option<bool>,
    pub avatar_hash: Option<String>,
    pub usernames: Option<Vec<ApiUsername>>,
    pub members_count: Option<i32>,
    pub join_date: Option<i32>,
    pub is_support: Option<bool>,
    pub photos: Option<Vec<ApiPhoto>>,
    pub draft_date: Option<i32>,
    pub is_protected: Option<bool>,
    pub fake_type: Option<ApiFakeType>,
    pub is_forum: Option<bool>,
    pub topics: Option<HashMap<i32, ApiTopic>>,
    pub listed_topic_ids: Option<Vec<i32>>,
    pub topics_count: Option<i32>,
    pub ordered_pinned_topic_ids: Option<Vec<i32>>,

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

    pub migrated_to: Option<ApiMigratedTo>,

    // Obtained from GetChatSettings
    pub settings: Option<ApiChatSettings>,
    // Obtained from GetFullChat / GetFullChannel
    pub full_info: Option<ApiChatFullInfo>,

    pub join_requests: Option<Vec<ApiChatInviteImporter>>,
    pub is_join_to_send: Option<bool>,
    pub is_join_request: Option<bool>,
    pub send_as_peer_ids: Option<Vec<ApiSendAsPeerId>>,

    pub unread_reactions: Option<Vec<i32>>,
    pub unread_mentions: Option<Vec<i32>>,
}

pub enum ApiChatType {
    ChatTypePrivate,
    ChatTypeSecret,
    ChatTypeBasicGroup,
    ChatTypeSuperGroup,
    ChatTypeChannel,
}

pub struct ApiMessage {
    // ...
}

pub struct ApiPhoto {
    // ...
}

pub struct ApiUsername {
    // ...
}

pub enum ApiFakeType {
    // ...
}

pub struct ApiTopic {
    // ...
}

pub struct ApiRestrictionReason {
    // ...
}

pub struct ApiChatAdminRights {
    // ...
}

pub struct ApiChatBannedRights {
    // ...
}

pub struct ApiMigratedTo {
    pub chat_id: String,
    pub access_hash: Option<String>,
}

pub struct ApiChatSettings {
    // ...
}

pub struct ApiChatFullInfo {
    // ...
}

pub struct ApiChatInviteImporter {
    // ...
}

pub struct ApiSendAsPeerId {
    // ...
}

pub struct ApiChatReactions {
    // ...
}

pub struct ApiBotCommand {
    // ...
}

pub struct ApiStickerSet {
    // ...
}
