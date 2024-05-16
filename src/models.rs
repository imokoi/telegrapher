use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParseMode {
    MarkdownV2,
    HTML,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    /// user id
    pub id: u64,
    /// if the user is a bot
    pub is_bot: bool,
    /// user first name
    pub first_name: String,
    /// user last name
    pub last_name: Option<String>,
    /// username
    pub username: Option<String>,
    /// user language code
    pub language_code: Option<String>,
    /// True, if this user is a Telegram Premium user
    pub is_premium: Option<bool>,
    /// True, if this user added the bot to the attachment menu
    pub added_to_attachment_menu: Option<bool>,
    /// True, if the bot can be invited to groups. Returned only in getMe.
    pub can_join_groups: Option<bool>,
    /// True, if privacy mode is disabled for the bot. Returned only in getMe.
    pub can_read_all_group_messages: Option<bool>,
    /// True, if the bot supports inline queries. Returned only in getMe.
    pub supports_inline_queries: Option<bool>,
    /// True, if the bot can be connected to a Telegram Business account to receive its messages. Returned only in getMe.
    pub can_connect_to_business: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub id: i64,
    /// Type of the chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub chat_type: String,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// True, if the supergroup chat is a forum (has topics enabled)
    pub is_forum: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub message_id: i32,
    /// Optional. Unique identifier of a message thread to which the message belongs; for supergroups only
    pub message_thread_id: Option<i32>,
    /// message sender
    pub from: Option<Box<User>>,
    /// sender chat
    pub sender_chat: Option<Box<Chat>>,
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<u32>,
    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    pub sender_business_bot: Option<Box<User>>,
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    pub date: u64,
    /// business_connection_id
    pub business_connection_id: Option<String>,
    /// Chat the message belongs to
    pub chat: Box<Chat>,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<Box<MessageOrigin>>,
    // is topic message
    pub is_topic_message: Option<bool>,
    /// is automatic forward
    pub is_automatic_forward: Option<bool>,
    /// reply_to_message
    pub reply_to_message: Option<Box<Message>>,
    #[serde(skip)]
    pub external_reply: Option<String>,
    /// message quote
    pub quote: Option<Box<TextQuote>>,
    /// For replies to a story, the original story
    pub reply_to_story: Option<Box<Story>>,
    /// Bot through which the message was sent
    pub via_bot: Option<Box<User>>,
    /// edit data
    pub edit_date: Option<u64>,
    /// if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    pub is_from_offline: Option<bool>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// animation
    pub animation: Option<Box<Animation>>,
    /// Message is an audio file, information about the file
    pub audio: Option<Box<Audio>>,
    /// Message is a general file, information about the file
    pub document: Option<Box<Document>>,
    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Box<Sticker>>,
    /// Message is a forwarded story
    pub story: Option<Box<Story>>,
    /// Message is a video, information about the video
    pub video: Option<Box<Video>>,
    /// Message is a video note, information about the video message
    pub video_note: Option<Box<VideoNote>>,
    /// Message is a voice message, information about the file
    pub voice: Option<Box<Voice>>,
    /// Caption for the animation, audio, document, photo, video or voice
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// True, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
    /// Message is a shared contact, information about the contact
    pub contact: Option<Box<Contact>>,
    /// Message is a dice with random value
    pub dice: Option<Box<Dice>>,
    /// Message is a game, information about the game
    pub game: Option<Box<Game>>,
    /// Message is a native poll, information about the poll
    pub poll: Option<Box<Poll>>,
    /// Message is a venue, information about the venue
    pub venue: Option<Box<Venue>>,
    /// Message is a shared location, information about the location
    pub location: Option<Box<Location>>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<Box<User>>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted
    pub delete_chat_photo: Option<bool>,
    /// Service message: the group has been created
    pub group_chat_created: Option<bool>,
    /// Service message: the supergroup has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// Service message: the channel has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    pub channel_chat_created: Option<bool>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub message_auto_delete_timer_changed: Option<Box<MessageAutoDeleteTimerChanged>>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,
    /// Message is an invoice for a payment, information about the invoice
    pub invoice: Option<Box<Invoice>>,
    /// Message is a service message about a successful payment, information about the payment
    pub successful_payment: Option<Box<SuccessfulPayment>>,
    /// Service message: users were shared with the bot
    pub users_shared: Option<Box<UsersShared>>,

    pub chat_shared: Option<Box<ChatShared>>,

    pub connected_website: Option<String>,

    pub write_access_allowed: Option<WriteAccessAllowed>,

    pub passport_data: Option<Box<PassportData>>,

    pub proximity_alert_triggered: Option<Box<ProximityAlertTriggered>>,

    pub boost_added: Option<Box<ChatBoostAdded>>,

    pub forum_topic_created: Option<Box<ForumTopicCreated>>,

    pub forum_topic_edited: Option<Box<ForumTopicEdited>>,

    pub forum_topic_closed: Option<Box<ForumTopicClosed>>,

    pub forum_topic_reopened: Option<Box<ForumTopicReopened>>,

    pub general_forum_topic_hidden: Option<Box<GeneralForumTopicHidden>>,

    pub general_forum_topic_unhidden: Option<Box<GeneralForumTopicUnhidden>>,

    pub giveaway_created: Option<GiveawayCreated>,

    pub giveaway: Option<Giveaway>,

    pub giveaway_winners: Option<GiveawayWinners>,

    pub giveaway_completed: Option<GiveawayCompleted>,

    pub video_chat_started: Option<Box<VideoChatStarted>>,

    pub video_chat_ended: Option<Box<VideoChatEnded>>,

    pub video_chat_scheduled: Option<Box<VideoChatScheduled>>,

    pub video_chat_participants_invited: Option<Box<VideoChatParticipantsInvited>>,

    pub web_app_data: Option<Box<WebAppData>>,

    pub reply_markup: Option<Box<InlineKeyboardMarkup>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageId {
    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub type_field: String,
    pub offset: u16,
    pub length: u16,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextQuote {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub position: u32,
    pub is_manual: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Story {
    pub chat: Chat,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberLeft {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMemberBanned {
    pub user: User,
    pub until_date: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoChatStarted {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoChatScheduled {
    pub start_date: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CallbackGame {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    User(MessageOriginUser),
    HiddenUser(MessageOriginHiddenUser),
    Chat(MessageOriginChat),
    Channel(MessageOriginChannel),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageOriginUser {
    pub date: u64,
    pub sender_user: User,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageOriginHiddenUser {
    pub date: u64,
    pub sender_user_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChat {
    pub date: u64,

    pub sender_chat: Chat,

    pub author_signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChannel {
    pub date: u64,
    pub chat: Chat,
    pub message_id: i32,
    pub author_signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkPreviewOptions {
    pub is_disabled: Option<bool>,
    pub url: Option<String>,
    pub prefer_small_media: Option<bool>,
    pub prefer_large_media: Option<bool>,
    pub show_above_text: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
    pub thumbnail: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: u32,
    pub duration: u32,
    pub thumbnail: Option<PhotoSize>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: u32,
    pub mime_type: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<u64>,
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dice {
    pub emoji: String,
    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PollOption {
    pub text: String,
    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollAnswer {
    pub poll_id: String,
    pub voter_chat: Option<Chat>,
    pub user: Option<Box<User>>,
    pub option_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: u32,
    pub is_closed: bool,
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub allows_multiple_answers: bool,
    pub correct_option_id: Option<u8>,
    pub explanation: Option<String>,
    pub explanation_entities: Option<Vec<MessageEntity>>,
    pub open_period: Option<u32>,
    pub close_date: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: Option<f64>,
    pub live_period: Option<u32>,
    pub heading: Option<u16>,
    pub proximity_alert_radius: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatBoostAdded {
    pub boost_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: u32,
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForumTopicClosed {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForumTopicReopened {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneralForumTopicHidden {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneralForumTopicUnhidden {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SharedUser {
    pub user_id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsersShared {
    pub request_id: i32,
    pub users: Vec<SharedUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatShared {
    pub request_id: i32,
    pub chat_id: i64,
    pub title: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WriteAccessAllowed {
    pub from_request: Option<bool>,
    pub web_app_name: Option<String>,
    pub from_attachment_menu: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoChatEnded {
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoChatParticipantsInvited {
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub web_app: Option<WebAppInfo>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    pub callback_game: Option<CallbackGame>,
    pub pay: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwitchInlineQueryChosenChat {
    pub query: Option<String>,
    pub allow_user_chats: Option<bool>,
    pub allow_bot_chats: Option<bool>,
    pub allow_group_chats: Option<bool>,
    pub allow_channel_chats: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<MaybeInaccessibleMessage>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub width: u32,
    pub height: u32,
    pub is_animated: bool,
    pub is_video: bool,
    pub thumbnail: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub premium_animation: Option<File>,
    pub mask_position: Option<MaskPosition>,
    pub custom_emoji_id: Option<String>,
    pub needs_repainting: Option<bool>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: u32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: u64,
    pub file_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebAppInfo {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SentWebAppMessage {
    pub inline_message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GiveawayCreated {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: u64,
    pub winner_count: u32,
    pub only_new_members: Option<bool>,
    pub has_public_winners: Option<bool>,
    pub prize_description: Option<String>,
    pub country_codes: Option<Vec<String>>,
    pub premium_subscription_month_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i32,
    pub winners_selection_date: u64,
    pub winner_count: u32,
    pub winners: Vec<User>,
    pub additional_chat_count: Option<u32>,
    pub premium_subscription_month_count: Option<u32>,
    pub unclaimed_prize_count: Option<u32>,
    pub only_new_members: Option<bool>,
    pub was_refunded: Option<bool>,
    pub prize_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GiveawayCompleted {
    pub winner_count: u32,
    pub unclaimed_prize_count: Option<u32>,
    pub giveaway_message: Option<Box<Message>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PassportElementErrorUnspecified {
    #[serde(rename = "type")]
    pub type_field: String,
    pub element_hash: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: i32,
    pub date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyParameters {
    pub message_id: i32,
    pub chat_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub quote: Option<TextQuote>,
    pub quote_parse_mode: Option<String>,
    pub quote_entities: Option<Vec<MessageEntity>>,
    pub quote_position: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "lowercase")]
// pub enum StickerFormat {
//     Static,
//     Animated,
//     Video,
// }

// // #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// // #[serde(untagged)]
// // pub enum InputMessageContent {
// //     Text(InputTextMessageContent),
// //     Location(InputLocationMessageContent),
// //     Venue(InputVenueMessageContent),
// //     Contact(InputContactMessageContent),
// //     Invoice(InputInvoiceMessageContent),
// // }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(tag = "status", rename_all = "lowercase")]
// pub enum ChatMember {
//     Creator(ChatMemberOwner),
//     Administrator(ChatMemberAdministrator),
//     Member(ChatMemberMember),
//     Restricted(ChatMemberRestricted),
//     Left(ChatMemberLeft),
//     Kicked(ChatMemberBanned),
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "lowercase")]
// pub enum ChatType {
//     Private,
//     Group,
//     Supergroup,
//     Channel,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum MessageEntityType {
//     Mention,
//     Hashtag,
//     Cashtag,
//     BotCommand,
//     Url,
//     Email,
//     PhoneNumber,
//     Bold,
//     Italic,
//     Underline,
//     Strikethrough,
//     Spoiler,
//     Code,
//     Pre,
//     TextLink,
//     TextMention,
//     CustomEmoji,
//     Blockquote,
//     #[serde(other)]
//     Unknown,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "lowercase")]
// pub enum PollType {
//     Regular,
//     Quiz,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorDataFieldType {
//     PersonalDetails,
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
//     Address,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorFrontSideType {
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorReverseSideType {
//     DriverLicense,
//     IdentityCard,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorSelfieType {
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorFileType {
//     UtilityBill,
//     BankStatement,
//     RentalAgreement,
//     PassportRegistration,
//     TemporaryRegistration,
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PassportElementErrorTranslationFileType {
//     Passport,
//     DriverLicense,
//     IdentityCard,
//     InternalPassport,
//     UtilityBill,
//     BankStatement,
//     RentalAgreement,
//     PassportRegistration,
//     TemporaryRegistration,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(tag = "type", rename_all = "snake_case")]
// pub enum MenuButton {
//     Commands,
//     WebApp(MenuButtonWebApp),
//     Default,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct MenuButtonWebApp {
//     pub text: String,
//     pub web_app: WebAppInfo,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatMemberOwner {
//     pub user: User,
//
//     pub custom_title: Option<String>,
//     pub is_anonymous: bool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatMemberAdministrator {
//     pub user: User,
//     pub can_be_edited: bool,
//     pub is_anonymous: bool,
//     pub can_manage_chat: bool,
//     pub can_delete_messages: bool,
//     pub can_manage_video_chats: bool,
//     pub can_restrict_members: bool,
//     pub can_promote_members: bool,
//     pub can_change_info: bool,
//     pub can_invite_users: bool,
//
//     pub can_post_messages: Option<bool>,
//
//     pub can_edit_messages: Option<bool>,
//
//     pub can_pin_messages: Option<bool>,
//
//     pub can_post_stories: Option<bool>,
//
//     pub can_edit_stories: Option<bool>,
//
//     pub can_delete_stories: Option<bool>,
//
//     pub can_manage_topics: Option<bool>,
//
//     pub custom_title: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatMemberMember {
//     pub user: User,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatMemberRestricted {
//     pub user: User,
//     pub is_member: bool,
//     pub can_send_messages: bool,
//     pub can_send_audios: bool,
//     pub can_send_documents: bool,
//     pub can_send_photos: bool,
//     pub can_send_videos: bool,
//     pub can_send_video_notes: bool,
//     pub can_send_voice_notes: bool,
//     pub can_send_polls: bool,
//     pub can_send_other_messages: bool,
//     pub can_add_web_page_previews: bool,
//     pub can_change_info: bool,
//     pub can_invite_users: bool,
//     pub can_pin_messages: bool,
//     pub can_manage_topics: bool,
//     pub until_date: u64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct BotDescription {
//     pub description: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct BotName {
//     pub name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct BotShortDescription {
//     pub short_description: String,
// }

// /// Represents an incoming update from telegram.
// /// [Official documentation.](https://core.telegram.org/bots/api#update)
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Update {
//     pub update_id: u32,

//     /// Maps to exactly one of the many optional fields
//     /// from [the official documentation](https://core.telegram.org/bots/api#update).
//     #[serde(flatten)]
//     pub content: UpdateContent,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(rename_all = "snake_case")]
// pub enum UpdateContent {
//     Message(Message),
//     // EditedMessage(Message),
//     // ChannelPost(Message),
//     // EditedChannelPost(Message),
//     // BusinessConnection(BusinessConnection),
//     // BusinessMessage(Message),
//     // EditedBusinessMessage(Message),
//     // DeletedBusinessMessages(BusinessMessagesDeleted),
//     // MessageReaction(MessageReactionUpdated),
//     // MessageReactionCount(MessageReactionCountUpdated),
//     // InlineQuery(InlineQuery),
//     // ChosenInlineResult(ChosenInlineResult),
//     // CallbackQuery(CallbackQuery),
//     // ShippingQuery(ShippingQuery),
//     // PreCheckoutQuery(PreCheckoutQuery),
//     // Poll(Poll),
//     // PollAnswer(PollAnswer),
//     // MyChatMember(ChatMemberUpdated),
//     // ChatMember(ChatMemberUpdated),
//     // ChatJoinRequest(ChatJoinRequest),
//     // ChatBoost(ChatBoost),
//     // RemovedChatBoost(ChatBoostRemoved),
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct WebhookInfo {
//     pub url: String,
//     pub has_custom_certificate: bool,
//     pub pending_update_count: u32,
//
//     pub ip_address: Option<String>,
//
//     pub last_error_date: Option<u64>,
//
//     pub last_error_message: Option<String>,
//
//     pub last_synchronization_error_date: Option<u64>,
//
//     pub max_connections: Option<u16>,
//
//     pub allowed_updates: Option<Vec<AllowedUpdate>>,
// }

// /// Control which updates to receive.
// /// Specify an empty list to receive all update types except `ChatMember`.
// /// [Official documentation](https://core.telegram.org/bots/api#getupdates).
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[non_exhaustive]
// pub enum AllowedUpdate {
//     Message,
//     EditedMessage,
//     ChannelPost,
//     EditedChannelPost,
//     MessageReaction,
//     MessageReactionCount,
//     InlineQuery,
//     ChosenInlineResult,
//     CallbackQuery,
//     ShippingQuery,
//     PreCheckoutQuery,
//     Poll,
//     PollAnswer,
//     MyChatMember,
//     ChatMember,
//     ChatJoinRequest,
//     ChatBoost,
//     RemovedChatBoost,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// pub struct ExternalReplyInfo {
//
//     pub origin: Option<MessageOrigin>,

//
//     pub chat: Option<Chat>,

//
//     pub message_id: Option<i32>,

//
//     pub link_preview_options: Option<LinkPreviewOptions>,

//
//     pub animation: Option<Animation>,

//
//     pub audio: Option<Audio>,

//
//     pub document: Option<Document>,

//
//     pub photo: Option<Vec<PhotoSize>>,

//
//     pub sticker: Option<Sticker>,

//
//     pub story: Option<Story>,

//
//     pub video: Option<Video>,

//
//     pub video_note: Option<VideoNote>,

//
//     pub voice: Option<Voice>,

//
//     pub has_media_spoiler: Option<bool>,

//
//     pub contact: Option<Contact>,

//
//     pub dice: Option<Dice>,

//
//     pub game: Option<Game>,

//
//     pub giveaway: Option<Giveaway>,

//
//     pub giveaway_winners: Option<GiveawayWinners>,

//
//     pub invoice: Option<Invoice>,

//
//     pub location: Option<Location>,

//
//     pub poll: Option<Poll>,

//
//     pub venue: Option<Venue>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct UserProfilePhotos {
//     pub total_count: u32,

//     pub photos: Vec<Vec<PhotoSize>>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub is_persistent: Option<bool>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButton {
    pub text: String,
    pub request_users: Option<KeyboardButtonRequestUsers>,
    pub request_chat: Option<KeyboardButtonRequestChat>,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
    pub request_poll: Option<KeyboardButtonPollType>,
    pub web_app: Option<WebAppInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i32,
    pub user_is_bot: Option<bool>,
    pub user_is_premium: Option<bool>,
    pub max_quantity: Option<u32>,
    pub request_name: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButtonRequestChat {
    pub request_id: i32,
    pub chat_is_channel: bool,
    pub chat_is_forum: Option<bool>,
    pub chat_has_username: Option<bool>,
    pub chat_is_created: Option<bool>,
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    pub bot_is_member: Option<bool>,
    pub request_title: Option<bool>,
    pub request_username: Option<bool>,
    pub request_photo: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForceReply {
    pub force_reply: bool,
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatPhoto {
//     pub small_file_id: String,

//     pub small_file_unique_id: String,

//     pub big_file_id: String,

//     pub big_file_unique_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatInviteLink {
//     pub invite_link: String,

//     pub creator: User,

//     pub creates_join_request: bool,

//     pub is_primary: bool,

//     pub is_revoked: bool,

//
//     pub name: Option<String>,

//
//     pub expire_date: Option<u64>,

//
//     pub member_limit: Option<u32>,

//
//     pub pending_join_request_count: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ChatMemberUpdated {
//     pub chat: Chat,

//     pub from: User,

//     pub date: u64,

//     pub old_chat_member: ChatMember,

//     pub new_chat_member: ChatMember,

//
//     pub invite_link: Option<ChatInviteLink>,

//
//     pub via_chat_folder_invite_link: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ChatJoinRequest {
//     pub chat: Chat,

//     pub from: User,

//     pub user_chat_id: u64,

//     pub date: u64,

//
//     pub bio: Option<String>,

//
//     pub invite_link: Option<ChatInviteLink>,
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatPermissions {
//
//     pub can_send_messages: Option<bool>,

//
//     pub can_send_audios: Option<bool>,

//
//     pub can_send_documents: Option<bool>,

//
//     pub can_send_photos: Option<bool>,

//
//     pub can_send_videos: Option<bool>,

//
//     pub can_send_video_notes: Option<bool>,

//
//     pub can_send_voice_notes: Option<bool>,

//
//     pub can_send_polls: Option<bool>,

//
//     pub can_send_other_messages: Option<bool>,

//
//     pub can_add_web_page_previews: Option<bool>,

//
//     pub can_change_info: Option<bool>,

//
//     pub can_invite_users: Option<bool>,

//
//     pub can_pin_messages: Option<bool>,

//
//     pub can_manage_topics: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct Birthdate {
//     pub day: u8,

//     pub month: u8,

//     pub year: u16,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct BusinessIntro {
//
//     pub title: Option<String>,

//
//     pub message: Option<String>,

//
//     pub sticker: Option<Sticker>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct BusinessLocation {
//     pub address: String,

//
//     pub location: Option<Location>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct BusinessOpeningHoursInterval {
//     pub opening_minute: u16,

//     pub closing_minute: u16,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct BusinessOpeningHours {
//     pub time_zone_name: String,

//     pub opening_hours: Vec<BusinessOpeningHoursInterval>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ChatLocation {
//     pub location: Location,

//     pub address: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(tag = "type", rename_all = "snake_case")]
// pub enum ReactionType {
//     Emoji(ReactionTypeEmoji),
//     CustomEmoji(ReactionTypeCustomEmoji),
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ReactionTypeEmoji {
//     pub emoji: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ReactionTypeCustomEmoji {
//     pub custom_emoji_id: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ReactionCount {
//     #[serde(rename = "type")]
//     pub type_field: ReactionType,

//     pub total_count: i32,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct MessageReactionUpdated {
//     pub chat: Chat,

//     pub message_id: i32,

//
//     pub user: Option<User>,

//
//     pub actor_chat: Option<Chat>,

//     pub date: u64,

//     pub old_reaction: Vec<ReactionType>,

//     pub new_reaction: Vec<ReactionType>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct MessageReactionCountUpdated {
//     pub chat: Chat,

//     pub message_id: i32,

//     pub date: u64,

//     pub reactions: Vec<ReactionCount>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
// pub struct ForumTopic {
//     pub message_thread_id: i32,

//     pub name: String,

//     pub icon_color: u32,

//
//     pub icon_custom_emoji_id: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct BotCommand {
//     pub command: String,

//     pub description: String,
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ResponseParameters {
//
//     pub migrate_to_chat_id: Option<i64>,

//
//     pub retry_after: Option<u16>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InputSticker {
//     pub sticker: FileUpload,
//     pub format: StickerFormat,
//     pub emoji_list: Vec<String>,

//
//     pub mask_position: Option<MaskPosition>,

//
//     pub keywords: Option<Vec<String>>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct StickerSet {
//     pub name: String,
//     pub title: String,
//     #[serde(rename = "sticker_type")]
//     pub sticker_type: StickerType,

//     #[doc(hidden)]
//     #[deprecated(since = "0.19.2", note = "Please use `sticker_type` instead")]
//     pub contains_masks: bool,

//     pub stickers: Vec<Sticker>,

//
//     pub thumbnail: Option<PhotoSize>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQuery {
//     pub id: String,

//     pub from: User,

//
//     pub location: Option<Location>,

//
//     pub chat_type: Option<String>,

//     pub query: String,

//     pub offset: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultArticle {
//     pub id: String,

//     pub title: String,

//     pub input_message_content: InputMessageContent,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub url: Option<String>,

//
//     pub hide_url: Option<bool>,

//
//     pub description: Option<String>,

//
//     pub thumbnail_url: Option<String>,

//
//     pub thumbnail_width: Option<u32>,

//
//     pub thumbnail_height: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultPhoto {
//     pub id: String,

//     pub photo_url: String,

//     pub thumbnail_url: String,

//
//     pub photo_width: Option<u32>,

//
//     pub photo_height: Option<u32>,

//
//     pub title: Option<String>,

//
//     pub description: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultGif {
//     pub id: String,

//     pub gif_url: String,

//
//     pub gif_width: Option<u32>,

//
//     pub gif_height: Option<u32>,

//
//     pub gif_duration: Option<u32>,

//     pub thumbnail_url: String,

//
//     pub thumbnail_mime_type: Option<String>,

//
//     pub title: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultMpeg4Gif {
//     pub id: String,

//     pub mpeg4_url: String,

//
//     pub mpeg4_width: Option<u32>,

//
//     pub mpeg4_height: Option<u32>,

//
//     pub mpeg4_duration: Option<u32>,

//     pub thumbnail_url: String,

//
//     pub thumbnail_mime_type: Option<String>,

//
//     pub title: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultVideo {
//     pub id: String,

//     pub video_url: String,

//     pub mime_type: String,

//     pub thumbnail_url: String,

//     pub title: String,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub video_width: Option<u32>,

//
//     pub video_height: Option<u32>,

//
//     pub video_duration: Option<u32>,

//
//     pub description: Option<String>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultAudio {
//     pub id: String,

//     pub audio_url: String,

//     pub title: String,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub performer: Option<String>,

//
//     pub audio_duration: Option<u32>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultVoice {
//     pub id: String,

//     pub voice_url: String,

//     pub title: String,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub voice_duration: Option<u32>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultDocument {
//     pub id: String,

//     pub title: String,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//     pub document_url: String,

//     pub mime_type: String,

//
//     pub description: Option<String>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,

//
//     pub thumbnail_url: Option<String>,

//
//     pub thumbnail_width: Option<u32>,

//
//     pub thumbnail_height: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultLocation {
//     pub id: String,

//     pub latitude: f64,

//     pub longitude: f64,

//     pub title: String,

//
//     pub horizontal_accuracy: Option<f64>,

//
//     pub live_period: Option<u32>,

//
//     pub heading: Option<u16>,

//
//     pub proximity_alert_radius: Option<u32>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,

//
//     pub thumbnail_url: Option<String>,

//
//     pub thumbnail_width: Option<u32>,

//
//     pub thumbnail_height: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultVenue {
//     pub id: String,

//     pub latitude: f64,

//     pub longitude: f64,

//     pub title: String,

//     pub address: String,

//
//     pub foursquare_id: Option<String>,

//
//     pub foursquare_type: Option<String>,

//
//     pub google_place_id: Option<String>,

//
//     pub google_place_type: Option<String>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,

//
//     pub thumbnail_url: Option<String>,

//
//     pub thumbnail_width: Option<u32>,

//
//     pub thumbnail_height: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultContact {
//     pub id: String,

//     pub phone_number: String,

//     pub first_name: String,

//
//     pub last_name: Option<String>,

//
//     pub vcard: Option<String>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,

//
//     pub thumbnail_url: Option<String>,

//
//     pub thumbnail_width: Option<u32>,

//
//     pub thumbnail_height: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct InlineQueryResultGame {
//     pub id: String,

//     pub game_short_name: String,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedPhoto {
//     pub id: String,

//     pub photo_file_id: String,

//
//     pub title: Option<String>,

//
//     pub description: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedGif {
//     pub id: String,

//     pub gif_file_id: String,

//
//     pub title: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedMpeg4Gif {
//     pub id: String,

//     pub mpeg4_file_id: String,

//
//     pub title: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedSticker {
//     pub id: String,

//     pub sticker_file_id: String,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedDocument {
//     pub id: String,

//     pub title: String,

//     pub document_file_id: String,

//
//     pub description: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedVideo {
//     pub id: String,

//     pub video_file_id: String,

//     pub title: String,

//
//     pub description: Option<String>,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedVoice {
//     pub id: String,

//     pub voice_file_id: String,

//     pub title: String,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InlineQueryResultCachedAudio {
//     pub id: String,

//     pub audio_file_id: String,

//
//     pub caption: Option<String>,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub caption_entities: Option<Vec<MessageEntity>>,

//
//     pub reply_markup: Option<InlineKeyboardMarkup>,

//
//     pub input_message_content: Option<InputMessageContent>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct InputTextMessageContent {
//     pub message_text: String,

//
//     pub parse_mode: Option<ParseMode>,

//
//     pub entities: Option<Vec<MessageEntity>>,

//
//     pub link_preview_options: Option<LinkPreviewOptions>,
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
// pub struct InputLocationMessageContent {
//     pub latitude: f64,

//     pub longitude: f64,

//
//     pub horizontal_accuracy: Option<f64>,

//
//     pub live_period: Option<u32>,

//
//     pub heading: Option<u16>,

//
//     pub proximity_alert_radius: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct InputInvoiceMessageContent {
//     pub title: String,

//     pub description: String,

//     pub payload: String,

//     pub provider_token: String,

//     pub currency: String,

//     pub prices: Vec<LabeledPrice>,

//
//     pub max_tip_amount: Option<u32>,

//
//     pub suggested_tip_amounts: Option<Vec<u32>>,

//
//     pub provider_data: Option<String>,

//
//     pub photo_url: Option<String>,

//
//     pub photo_size: Option<u32>,

//
//     pub photo_width: Option<u32>,

//
//     pub photo_height: Option<u32>,

//
//     pub need_name: Option<bool>,

//
//     pub need_phone_number: Option<bool>,

//
//     pub need_email: Option<bool>,

//
//     pub need_shipping_address: Option<bool>,

//
//     pub send_phone_number_to_provider: Option<bool>,

//
//     pub send_email_to_provider: Option<bool>,

//
//     pub is_flexible: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct InputVenueMessageContent {
//     pub latitude: f64,

//     pub longitude: f64,

//     pub title: String,

//     pub address: String,

//
//     pub foursquare_id: Option<String>,

//
//     pub foursquare_type: Option<String>,

//
//     pub google_place_id: Option<String>,

//
//     pub google_place_type: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct InputContactMessageContent {
//     pub phone_number: String,

//     pub first_name: String,

//
//     pub last_name: Option<String>,

//
//     pub vcard: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ChosenInlineResult {
//     pub result_id: String,

//     pub from: User,

//
//     pub location: Option<Location>,

//
//     pub inline_message_id: Option<String>,

//     pub query: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ShippingQuery {
//     pub id: String,

//     pub from: User,

//     pub invoice_payload: String,

//     pub shipping_address: ShippingAddress,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PreCheckoutQuery {
//     pub id: String,

//     pub from: User,

//     pub currency: String,

//     pub total_amount: u32,

//     pub invoice_payload: String,

//
//     pub shipping_option_id: Option<String>,

//
//     pub order_info: Option<OrderInfo>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorDataField {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorDataFieldType,

//     pub field_name: String,

//     pub data_hash: String,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorFrontSide {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorFrontSideType,

//     pub file_hash: String,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorReverseSide {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorReverseSideType,

//     pub file_hash: String,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorSelfie {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorSelfieType,

//     pub file_hash: String,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorFile {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorFileType,

//     pub file_hash: String,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorFiles {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorFileType,

//     pub file_hashes: Vec<String>,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorTranslationFile {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorTranslationFileType,

//     pub file_hash: String,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct PassportElementErrorTranslationFiles {
//     #[serde(rename = "type")]
//     pub type_field: PassportElementErrorTranslationFileType,

//     pub file_hashes: Vec<String>,

//     pub message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct GameHighScore {
//     pub position: u32,

//     pub user: User,

//     pub score: i32,
// }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatAdministratorRights {
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_post_stories: Option<bool>,
    pub can_edit_stories: Option<bool>,
    pub can_delete_stories: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(tag = "source", rename_all = "snake_case")]
// pub enum ChatBoostSource {
//     Premium(ChatBoostSourcePremium),
//     GiftCode(ChatBoostSourceGiftCode),
//     Giveaway(ChatBoostSourceGiveaway),
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatBoostSourcePremium {
//     pub user: User,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatBoostSourceGiftCode {
//     pub user: User,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatBoostSourceGiveaway {
//     pub giveaway_message_id: i32,

//
//     pub user: Option<User>,

//
//     pub is_unclaimed: Option<bool>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct ChatBoost {
//     pub boost_id: String,

//     pub add_date: u64,

//     pub expiration_date: u64,

//     pub source: ChatBoostSource,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ChatBoostUpdated {
//     pub chat: Chat,

//     pub boost: ChatBoost,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ChatBoostRemoved {
//     pub chat: Chat,

//     pub boost_id: String,

//     pub remove_date: u64,

//     pub source: ChatBoostSource,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct UserChatBoosts {
//     pub boosts: Vec<ChatBoost>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct BusinessConnection {
//     pub id: String,

//     pub user: User,

//     pub user_chat_id: u64,

//     pub date: u64,

//     pub can_reply: bool,

//     pub is_enabled: bool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct BusinessMessagesDeleted {
//     pub business_connection_id: String,

//     pub chat: Chat,

//     pub message_ids: Vec<i32>,
// }
