use std::io;
use std::time::Duration;
use std::str;

use clap::{value_parser, Arg, Command};
use serenity::all::{ChannelId, Context, CreateMessage};

use crate::{commands, types::EmbedNavigatorKey};

async fn create_embed(ctx: Context, channel_id: ChannelId) {

    let embeds = commands::fmad::run();
    let embed = &embeds[0];
    let builder = CreateMessage::new().embed(embed.clone());
    /* let Ok(message_id), Err(why) = channel_id.send_message(&ctx.http, builder).await;
    if let Ok(message_id Err(why) =  {
        println!("Cannot respond to slash command: {why}");
    } */
    match channel_id.send_message(&ctx.http, builder).await {
        Ok(message) => {
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
        },
        Err(why) => { println!("Cannot respond to slash command: {why}"); }
    }
}

pub async fn check_fridge_open(ctx: Context, channel_id: ChannelId) {
    let matches = Command::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
                .use_value_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to connect at")
                .use_value_delimiter(false)
                .required(true)
                .value_parser(value_parser!(u32)),
        )
        .get_matches();

    let port_name = matches.get_one::<String>("port").unwrap();
    let baud_rate = *matches.get_one::<u32>("baud").unwrap();
    println!("port_name: {port_name}");
    println!("baud_rate: {baud_rate}");

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        let data = str::from_utf8(&serial_buf[..t]).unwrap();
                        match data {
                            "The fridge is open!" => {
                                create_embed(ctx.clone(), channel_id).await;
                            },
                            _ => ()
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(_) => (),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
