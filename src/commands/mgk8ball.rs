use serenity::
{
    prelude::*,
    model::prelude::*,
    framework::standard::{ CommandResult, macros::command },
};
use rand::Rng;
//use std::{thread, time};

#[command]
#[aliases(m8ball, helpmelisa, whatdo, ask)]
async fn asklisa(ctx: &Context, msg: &Message) -> CommandResult
{
    let roll = rand::thread_rng().gen_range(0 as i64..2000);

    if roll > 1800
    {
        &msg.channel_id.say(&ctx.http, format!("Yes! ({})", roll)).await?;
    }
    else if roll > 1600
    {
        &msg.channel_id.say(&ctx.http, format!("It is certain. ({})", roll)).await?;
    }
    else if roll > 1400
    {
        &msg.channel_id.say(&ctx.http, format!("Obviously. ({})", roll)).await?;
    }
    else if roll > 1200
    {
        &msg.channel_id.say(&ctx.http, format!("Probably... maybe. ({})", roll)).await?;
    }
    else if roll > 1000
    {
        &msg.channel_id.say(&ctx.http, format!("How am I supposed to know? ({})", roll)).await?;
    }
    else if roll > 800
    {
        &msg.channel_id.say(&ctx.http, format!("I don't think so. ({})", roll)).await?;
    }
    else if roll > 600
    {
        &msg.channel_id.say(&ctx.http, format!("No. ({})", roll)).await?;
    }
    else if roll > 400
    {
        &msg.channel_id.say(&ctx.http, format!("Never ever. ({})", roll)).await?;
    }
    else if roll > 200
    {
        &msg.channel_id.say(&ctx.http, format!("Absolutely not. ({})", roll)).await?;
    }
    else if roll > 100
    {
        &msg.channel_id.say(&ctx.http, format!("How about no? ({})", roll)).await?;
    }
    else if roll > 50
    {
        &msg.channel_id.say(&ctx.http, format!("Maybe after I hit this fat vape. ({})", roll)).await?;
    }
    else if roll > 25
    {
        &msg.channel_id.say(&ctx.http, format!("Maybe after I hit this fat vape. *blows cloud* ({})", roll)).await?;
    }
    else if roll > 12
    {
        &msg.channel_id.say(&ctx.http, format!("*Starts aggressively dabbing to assert dominance* ({})", roll)).await?;
    }
    else if roll > 5
    {
        &msg.channel_id.say(&ctx.http, format!("OwO What's this? *Notices your qwestion* Rawr X3 *nuzzles* How are you? *pounces on you* you're so warm o3o *notices you have a bulge* someone's happy! *nuzzles your necky wecky* ~murr~ ({})", roll)).await?;
    }
    else if roll > 1
    {
        &msg.channel_id.say(&ctx.http, format!("Pebis. ({})", roll)).await?;
    }
    else
    {
        &msg.channel_id.say(&ctx.http, format!("https://www.youtube.com/watch?v=HS3hxgREKKw ({})", roll)).await?;
    }

    Ok(())
}