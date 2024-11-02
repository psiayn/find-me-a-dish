use serenity::all::Colour;
use serenity::builder::{CreateCommand, CreateEmbed};
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .title("Pong!")
        .description("This is an embedded message in response to /ping.")
        .colour(Colour::from_rgb(0, 255, 0))
        .field("Field 1", "Some interesting data", true)
        .field("Field 2", "More data here", true)
        .timestamp(chrono::Utc::now());
    embed
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
