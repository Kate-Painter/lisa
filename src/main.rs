use std::{path::Path, fs::File, {collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc}};
use std::io::Write as _write;
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        buckets::{RevertBucket, LimitedFor},
        macros::{command, group, help, check, hook},
    },
    http::{Http, AttachmentType},
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
};

use serenity::prelude::*;
use tokio::sync::Mutex;

mod commands;

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} online", ready.user.name);
    }
}

#[group]
#[summary = "General Commands"]
#[commands(about, commands, palette)]
struct General;

#[group]
#[owners_only]
#[only_in(guilds)]
#[summary = "Owner commands"]
#[commands(shutdown, log, latency)]
struct Owner;

#[help]
#[individual_command_tip = "For more information about a specific command, pass the command as an argument.\n"]
#[command_not_found_text = "We don't have any '{}' here."]
#[max_levenshtein_distance(2)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, &help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool
{
    println!("Received command '{}' from '{}'", command_name, msg.author.name);

    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult)
{
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, msg: &Message, unknown_command_name: &str) {
    println!("User '{}' couldn't find command '{}'", msg.author.name, unknown_command_name);
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    
    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .on_mention(Some(bot_id))
                   .prefix(">")
                   .delimiters(vec![", ", ","])
                   .owners(owners))
        .before(before)
        .after(after)
        .unrecognised_command(unknown_command)
        .bucket("palette", |b| b.delay(10)).await
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client");

    {
        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
    }

    if let Err(why) = client.start().await
    {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult
{
    let mut contents = "Commands used:\n".to_string();

    let data = ctx.data.read().await;
    let counter = data.get::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");

    for (k, v) in counter
    {
        writeln!(contents, "   >{name}: {amount}", name=k, amount=v)?;
    }

    msg.channel_id.say(&ctx.http, &contents).await?;
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(&ctx.http, "I am incompetent").await?;

    Ok(())
}

#[command]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(&ctx.http, "Byeeeeee").await?;
    Ok(())
}

#[command]
async fn palette(ctx: &Context, msg: &Message) -> CommandResult
{
    let dirname = "palette";
    for attachment in &msg.attachments
    {
        let inpath  = format!("./{}/{}", &dirname, &attachment.filename);

        let content = match attachment.download().await {
            Ok(content) => content,
            Err(why) => {
                println!("Failed to download attachment: ({:?})", why);
                msg.channel_id.say(&ctx.http, "Failed to download file").await?;

                return Ok(());
            }
        };

        let mut file = match File::create(&inpath)
        {
            Ok(file) => file,
            Err(why) => {
                println!("Failed to create file: ({:?})", why);
                msg.channel_id.say(&ctx.http, "Failed to create file").await?;

                return Ok(());
            },
        };

        if let Err(why) = file.write(&content) {
            println!("Error writing to file: {:?}", why);

            return Ok(());
        }

        let outfile = commands::palette::palettify_image(&inpath);
        let outpath = format!("{}", &outfile);

        let _msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.add_file(AttachmentType::Path(Path::new(&outpath)));
            m
        }).await;
    }

    Ok(())
}

#[command]
async fn log(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(&ctx.http, "I am incompetent").await?;

    Ok(())
}

#[command]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult
{
    msg.channel_id.say(&ctx.http, "I am incompetent").await?;

    Ok(())
}