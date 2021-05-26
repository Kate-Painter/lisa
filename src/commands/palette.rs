use std::path::Path;
use std::fs::File;
use std::time::SystemTime;
use std::io::Write as _write;
use image::GenericImageView;
use rand::Rng;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::http::AttachmentType;
use serenity::framework::standard::
{
    CommandResult,
    macros::command,
};

// TODO: Most of these should be user defined -- Defaults?
const BORDER_WIDTH: u32 = 5;
const NUM_COLORS: u32 = 8;  
const NUM_PER_COL:   u32 = 4;
const HEIGHT_FACTOR: f32 = 1.25;
const DIRNAME: &str = "palette";

#[command]
async fn palette(ctx: &Context, msg: &Message) -> CommandResult
{
    for attachment in &msg.attachments
    {
        let inpath  = format!("./{}/{}", &DIRNAME, &attachment.filename);

        let content = match attachment.download().await
        {
            Ok(content) => content,
            Err(why) =>
            {
                println!("Failed to download attachment: ({:?})", why);
                msg.channel_id.say(&ctx.http, "Failed to download file").await?;

                return Ok(());
            }
        };

        let mut file = match File::create(&inpath)
        {
            Ok(file) => file,
            Err(why) =>
            {
                println!("Failed to create file: ({:?})", why);
                msg.channel_id.say(&ctx.http, "Failed to create file").await?;

                return Ok(());
            },
        };

        if let Err(why) = file.write(&content)
        {
            println!("Error writing to file: {:?}", why);

            return Ok(());
        }

        let outfile = palettify_image(&inpath);
        let outpath = format!("{}", &outfile);

        let _msg = msg.channel_id.send_message(&ctx.http, |m|
        {
            m.add_file(AttachmentType::Path(Path::new(&outpath)));
            m
        }).await;
    }

    Ok(())
}

pub fn palettify_image(path: &str) -> String
{
    let input = image::open(&Path::new(&path)).expect("Bad path");

    // Set up useful information
    let buffer_w = input.width();
    let buffer_h = (input.height() as f32 * HEIGHT_FACTOR) as u32;
    let palette_h = buffer_h - input.height() - BORDER_WIDTH;
    let palette_loc = input.height() + BORDER_WIDTH;

    // Create new buffer and copy old image over
    let mut imgbuf = image::ImageBuffer::new(buffer_w, buffer_h);

    for x in 0..input.width()
    {
        for y in 0..input.height()
        {
            imgbuf.put_pixel(x, y, input.get_pixel(x, y).clone());
        }
    }

    let mut num_rows = NUM_COLORS / NUM_PER_COL;
    if NUM_COLORS % NUM_PER_COL != 0 { num_rows += 1 };

    let block_width = buffer_w / NUM_PER_COL;
    let block_height = palette_h / num_rows;
    
    for n in 0..NUM_COLORS
    {
        let color = sample_color(&input);
        for x in (0 + block_width * (n % NUM_PER_COL))..(0 + block_width * ((n % NUM_PER_COL) + 1))
        {
            for y in (palette_loc + block_height * (n / NUM_PER_COL))..(palette_loc + block_height * ((n / NUM_PER_COL) + 1))
            {
                imgbuf.put_pixel(x, y, color);
            }
        }
    }


    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
    {
        Ok(n) =>
        {  
            let name = format!("./palette/{}.png", n.as_secs());
            imgbuf.save(&name).expect("Problem saving result.");

            return name; 
        },
        
        Err(_) => panic!("Unable to find timestamp."),
    }
}

fn sample_color(image: &image::DynamicImage) -> image::Rgba<u8>
{
    let mut rng = rand::thread_rng();
    let (mut x, mut y): (u32, u32);

    loop
    {
        x = rng.gen_range(0..image.width()); 
        y = rng.gen_range(0..image.height());
        
        if image.get_pixel(x, y)[3] != 0 { break; }
    } 

    return image.get_pixel(x, y);
}