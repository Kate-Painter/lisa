use serenity::
{
    prelude::*,
    model::prelude::*,
    framework::standard::{ CommandResult, Args, macros::command },
};

#[command]
async fn color(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{ 
    Ok(())
}