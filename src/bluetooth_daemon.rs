use std::io;
use std::str;
use std::time::Duration;

use serenity::all::{ChannelId, Context, CreateMessage};

use crate::{commands, types::EmbedNavigatorKey};
use tokio::{runtime::Handle, task::spawn_blocking};
use log::{info, error};

async fn create_embed(ctx: Context, channel_id: ChannelId) {
    let embeds = commands::fmad::run();
    let embed = &embeds[0];
    let builder = CreateMessage::new().embed(embed.clone());
    match channel_id.send_message(&ctx.http, builder).await {
        Ok(message) => {
            let message_id = message.id;
            let mut data = ctx.data.write().await;
            let mut tracker = data
                .get_mut::<EmbedNavigatorKey>()
                .expect("Expected EmbedNavigator in TypeMap")
                .lock()
                .await;

            info!("Got mutex lock");

            tracker.embed_index.insert(message_id, 0);
            tracker.embeds.insert(message_id, embeds);
            
            info!("Created embed");

            message.react(&ctx.http, '👈').await.unwrap();
            message.react(&ctx.http, '👉').await.unwrap();
        }
        Err(why) => {
            error!("Cannot create embed from fridge: {why}");
        }
    }
    info!("Created embed and reacted to it");
}

pub async fn check_fridge_open(ctx: Context, channel_id: ChannelId) {
    let port_name = "/dev/rfcomm0";
    let baud_rate = 112500;
    println!("port_name: {port_name}");
    println!("baud_rate: {baud_rate}");

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();


    match port {
        Ok(mut port) => {
            let handle = Handle::current();
            spawn_blocking(move || {
                let mut serial_buf: Vec<u8> = vec![0; 1000];
                info!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
                loop {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => {
                            let data = str::from_utf8(&serial_buf[..t]).unwrap().trim();
                            println!("{data}");
                            match data {
                                "OPEN!" => {
                                    info!("Fridge is open");
                                    handle.block_on(async {
                                        create_embed(ctx.clone(), channel_id).await
                                    });
                                }
                                _ => ()
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            error!("Error timeout: {e}");
                        },
                        Err(e) => {
                            error!("Error while reading data from port: {e}");
                        }
                    }
                }
            })
            .await
            .unwrap();
        }
        Err(e) => {
            error!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
