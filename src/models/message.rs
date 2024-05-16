use serde::{Deserialize, Serialize};

use crate::models::animation::Animation;
use crate::models::audio::Audio;
use crate::models::boost::ChatBoostAdded;
use crate::models::contact::Contact;
use crate::models::dice::Dice;
use crate::models::document::Document;
use crate::models::forum::{
    ChatShared, ForumTopicClosed, ForumTopicCreated, ForumTopicEdited, ForumTopicReopened,
    GeneralForumTopicHidden, GeneralForumTopicUnhidden, UsersShared, VideoChatScheduled,
    VideoChatStarted, WriteAccessAllowed,
};
use crate::models::game::Game;
use crate::models::giveaway::{Giveaway, GiveawayCompleted, GiveawayCreated, GiveawayWinners};
use crate::models::invoice::Invoice;
use crate::models::link_preview::LinkPreviewOptions;
use crate::models::location::Location;
use crate::models::message_entity::MessageEntity;
use crate::models::passport_data::PassportData;
use crate::models::photo_size::PhotoSize;
use crate::models::poll::Poll;
use crate::models::proximity_alert_trigger::ProximityAlertTriggered;
use crate::models::reply::{InlineKeyboardMarkup, MessageOrigin, TextQuote};
use crate::models::sticker::Sticker;
use crate::models::story::Story;
use crate::models::successful_payment::SuccessfulPayment;
use crate::models::venue::Venue;
use crate::models::video::Video;
use crate::models::video_chat::{VideoChatEnded, VideoChatParticipantsInvited};
use crate::models::video_note::VideoNote;
use crate::models::voice::Voice;
use crate::models::web_app::WebAppData;

use super::{chat::Chat, user::User};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageId {
    pub message_id: i32,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: u32,
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
