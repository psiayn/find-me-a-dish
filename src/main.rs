mod bluetooth_daemon;
mod commands;
mod init;
mod rank;
mod types;

use std::collections::HashMap;
use std::env;

use bluetooth_daemon::check_fridge_open;
use dotenv::dotenv;

use serenity::all::ChannelId;
use serenity::all::{EditMessage, GuildId};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use types::{EmbedNavigator, EmbedNavigatorKey};
use log::{info, error};


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "ping" => {
                    let embed = commands::ping::run(&command.data.options());
                    let data = CreateInteractionResponseMessage::new().embed(embed);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        error!("Cannot respond to slash command: {why}");
                    }
                }
                "fmad" => {
                    let embeds = commands::fmad::run();
                    let embed = &embeds[0];
                    let data = CreateInteractionResponseMessage::new().embed(embed.clone());
                    let builder = CreateInteractionResponse::Message(data);
                    info!("FMAD invoked");

                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        error!("Cannot respond to slash command: {why}");
                    }

                    if let Ok(message) = command.get_response(&ctx.http).await {
                        let message_id = message.id;

                        let mut data = ctx.data.write().await;
                        let mut tracker = data
                            .get_mut::<EmbedNavigatorKey>()
                            .expect("Expected EmbedNavigator in TypeMap")
                            .lock()
                            .await;

                        tracker.embed_index.insert(message_id, 0);
                        tracker.embeds.insert(message_id, embeds);

                        message.react(&ctx.http, '👈').await.unwrap();
                        message.react(&ctx.http, '👉').await.unwrap();
                        info!("fmad");
                    }
                }
                _ => {}
            };
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let message_id = reaction.message_id;
        let user_id = reaction.user_id.unwrap();

        // Ignore bot's own reactions
        if user_id == ctx.cache.current_user().id {
            return;
        }

        // Access the state to check the current index for this message
        let mut data = ctx.data.write().await;
        let mut tracker = data
            .get_mut::<EmbedNavigatorKey>()
            .expect("Expected EmbedTracker in TypeMap.")
            .lock()
            .await;

        let mut current_index = *tracker.embed_index.entry(message_id).or_insert(0);
        let max_index = tracker.embeds.len().saturating_sub(1);

        info!("Inside reaction {}", reaction.emoji.to_string().as_str());

        // Update index based on reaction
        match reaction.emoji.to_string().as_str() {
            "👉" => {
                if current_index <= max_index {
                    current_index += 1;
                }
            }
            "👈" => {
                if current_index > 0 {
                    current_index -= 1;
                }
            }
            _ => {
                return;
            }
        }

        match tracker.embeds.get(&message_id) {
            Some(embeds) => {
                let builder = EditMessage::new().embed(embeds[current_index].clone());
                let _ = reaction
                    .channel_id
                    .edit_message(&ctx.http, message_id, builder)
                    .await;

                // Remove the reaction to reset for the next interaction
                let _ = reaction.delete(&ctx.http).await;
            }
            None => {
                println!("{:?}", tracker.embeds);
                return;
            }
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        let channel_id = ChannelId::new(
            env::var("CHANNEL_ID")
                .expect("Expected CHANNEL_ID in environment")
                .parse()
                .expect("CHANNEL_ID must be an integer"),
        );
        let ctx_clone = ctx.clone();

        guild_id
            .set_commands(
                &ctx.http,
                vec![commands::ping::register(), commands::fmad::register()],
            )
            .await
            .unwrap();

        tokio::spawn(async move {
            check_fridge_open(ctx_clone, channel_id).await;
        });

        println!("BAI");

    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    // thread::spawn(|| bluetooth_daemon::find_fridge_open);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<EmbedNavigatorKey>(Mutex::new(EmbedNavigator {
            embed_index: HashMap::new(),
            embeds: HashMap::new(),
        }));
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
