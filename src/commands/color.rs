use serenity::
{
    utils::Colour,
    prelude::*,
    model::prelude::*,
    framework::standard::{ CommandResult, Args, macros::command },
};
use hex;

#[command]
async fn color(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
    let arg_string = match args.single::<String>()
    {
        Ok(r) => r,
        Err(_) => "".to_string(),
    };

    let decoded_color = hex::decode(&arg_string).expect("Failed to decode color from hex string.");

    if arg_string != ""
    {
        let guild = match msg.guild(&ctx).await.ok_or(0)
        {
            Ok(g) => g,
            Err(_) => panic!(),
        };
    
        let mut highest_role = -1;
        let mut target_role: Role = ctx.http.get_guild_roles(*guild.id.as_u64()).await?[0].clone();
    
        for role in guild.roles
        {
            if msg.author.has_role(&ctx, guild.id, role.0).await?
            {
                if role.1.position > highest_role
                {
                    highest_role = role.1.position;
                    target_role = role.1;
                }
            }
        }
    
        if target_role != ctx.http.get_guild_roles(*guild.id.as_u64()).await?[0].clone()
        {
            target_role.edit(&ctx, |r| {
                r.colour(Colour::from_rgb(decoded_color[0], decoded_color[1], decoded_color[2]).0 as u64);

                r
            }).await?;
        }
        
        msg.react(&ctx, '✅').await?;
    }
    else
    {
        // TODO tell user they suck
        panic!();
    }

    Ok(())
}