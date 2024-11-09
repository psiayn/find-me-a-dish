use std::collections::HashMap;

use serenity::{all::{CreateEmbed, MessageId}, prelude::TypeMapKey};
use tokio::sync::Mutex;

pub struct EmbedNavigator {
    pub embed_index: HashMap<MessageId, usize>,
    pub embeds: HashMap<MessageId, Vec<CreateEmbed>>,
}

pub struct EmbedNavigatorKey;

impl TypeMapKey for EmbedNavigatorKey {
    type Value = Mutex<EmbedNavigator>;
}
