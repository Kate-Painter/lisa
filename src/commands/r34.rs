use std::{thread, time};
use select::{document::Document, predicate::{Class, Name}};
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::command,
};



#[command]
async fn r34(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
    const ARGS_REQUIRED: &str = "Arguments required for >r34";
    const NO_RESULTS:    &str = "No results found for keywords(s)";

    let arg_string = match args.single::<String>()
    {
        Ok(r) => r,
        Err(_) => 
        { 
            &msg.channel_id.say(&ctx, &ARGS_REQUIRED).await?;
            return Ok(());
        }
    };

    let keywords = arg_string.clone();
    let mut page: u32 = 1;

    let mut results = tokio::task::spawn_blocking(move||
    {
        return fetch_direct_links(&keywords, page);
    }).await?;

        if results.len() == 0
        {
            &msg.channel_id.say(&ctx, &NO_RESULTS).await?;
        }
        else
        {
            thread::sleep(time::Duration::new(1,0));
            
            let msg = msg.channel_id.send_message(&ctx.http, |m|
            {
                m.embed(|e|
                {
                    e.title(format!("R34: {}", &arg_string));
                    e.image(&results[0]);
                    e.color((150,235,252));
                    e.footer(|f|
                    {
                        f.text(format!("Image {}/{}             Page {}/{}", 1, results.len(), page, 10));
                        
                        f
                    });

                    e
                });
                
                m.reactions(vec![ReactionType::from('◀'),
                                 ReactionType::from('▶'),
                                 ReactionType::from('🔽'),
                                 ReactionType::from('🔼'),].into_iter());
                
                m
            }).await;

            let mut message = msg.unwrap();
            let mut current_image = 0;

            thread::sleep(time::Duration::new(0,500000000));
    
            loop
            {
                if let Some(r) = &message.await_reaction(ctx)
                    .collect_limit(1)
                    .message_id(message.id.as_u64().clone())
                    .timeout(tokio::time::Duration::from_secs(90))
                    .await
                {

                    let emoji = &r.as_inner_ref().emoji;
                    r.as_inner_ref().delete(&ctx).await?;

                    let _ = match emoji.as_data().as_str()
                    {
                        "◀" =>
                        { 
                            if current_image == 0
                            { 
                                current_image = results.len() - 1;
                            }
                            else
                            {
                                current_image = ((current_image - 1) as usize) % (results.len() - 1);
                            }

                            message.edit(&ctx, |m|
                            {
                                m.embed(|e|
                                {
                                    e.title(format!("R34: {}", &arg_string));
                                    e.image(&results[current_image]);
                                    e.color((150,235,252));
                                    e.footer(|f|
                                    {
                                        f.text(format!("Image {}/{}             Page {}/{}", current_image + 1, results.len(), page, 10));
                                        
                                        f
                                    });

                                    e
                                });

                                m
                                }).await?;
                        },
                        "▶" =>
                        { 
                            current_image = ((current_image + 1) as usize) % (results.len() - 1);
                            message.edit(&ctx, |m|
                            {
                                m.embed(|e|
                                    {
                                    e.title(format!("R34: {}", &arg_string));
                                    e.image(&results[current_image]);
                                    e.color((150,235,252));
                                    e.footer(|f|
                                    {
                                        f.text(format!("Image {}/{}             Page {}/{}", current_image + 1, results.len(), page, 10));
                                    
                                        f
                                    });

                                    e
                                });

                                m
                            }).await?;
                        },
                        "🔽" =>
                        { 
                            if page > 0 { page -= 1; }
                            current_image = 0;

                            let keywords = arg_string.clone();
                            results = tokio::task::spawn_blocking(move||
                            {
                                return fetch_direct_links(&keywords, page);
                            }).await?;

                            message.edit(&ctx, |m|
                            {
                                m.embed(|e|
                                    {
                                    e.title(format!("R34: {}", &arg_string));
                                    e.image(&results[current_image]);
                                    e.color((150,235,252));
                                    e.footer(|f|
                                    {
                                        f.text(format!("Image {}/{}             Page {}/{}", current_image + 1, results.len(), page, 10));
                                    
                                        f
                                    });

                                    e
                                });

                                m
                            }).await?;
                        },
                        "🔼" =>
                        { 
                            if page < 5 { page += 1; } else { page = 1 }
                            current_image = 0;

                            let keywords = arg_string.clone();
                            results = tokio::task::spawn_blocking(move||
                            {
                                return fetch_direct_links(&keywords, page);
                            }).await?;

                            message.edit(&ctx, |m|
                            {
                                m.embed(|e|
                                    {
                                    e.title(format!("R34: {}", &arg_string));
                                    e.image(&results[current_image]);
                                    e.color((150,235,252));
                                    e.footer(|f|
                                    {
                                        f.text(format!("Image {}/{}             Page {}/{}", current_image + 1, results.len(), page, 10));
                                    
                                        f
                                    });

                                    e
                                });

                                m
                            }).await?;
                        },
                        _  => { &message.channel_id.say(&ctx.http, "I'll cut you if you do that again.").await?; },
                    };
                }
                else
                {
                    &message.delete_reactions(&ctx).await?;
                    break;
                }
            }
        }
    Ok(())
}

/**
 *  Returns Vec<&str> of direct links to results of a rule34.paheal.net search using the provided space delimited keywords
 */
pub fn fetch_direct_links(keywords: &str, page: u32) -> Vec<String>
{
    // Create search request
    let page: String = format!("https://rule34.paheal.net/post/list/{}/{}", &keywords, &page);

    // Send request and hold response
    let response = reqwest::blocking::get(page).unwrap();
    if !response.status().is_success() { return Vec::new() };

    // Create searchable html document from response
    let html = Document::from_read(response).unwrap();

    let mut links: Vec<String> = Vec::new();
    for image_list in html.find(Class("shm-image-list"))
    {
        for image in image_list.find(Name("a"))
        {
            let results = image.attr("href").unwrap();
            if results.contains("https")
            {
                links.push(results.to_string());
            }
        }
    }

    return links;
}