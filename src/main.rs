use image::{GenericImageView, imageops::FilterType};
use std::env;

fn get_image(dir: &str, width: u32, height: u32) {
    let chars = ['@','M','B','H','E','N','R','#','K','W','X','D','F','P','Q','A','S','U','Z','b','d','e','h','x','*','8','G','m','&','0','4','L','O','V','Y','k','p','q','5','T','a','g','n','s','6','9','o','w','z','$','C','I','u','2','3','J','c','f','r','y','%','1','v','7','l','+','i','t','[',']','{','}','?','j','|','(',')','=','~','!','-','/','<','>','"','^','_','\'',';',',',':','`','.',' ',];
    let div: f32 = 256f32 / chars.len() as f32;
    let height = (height as f32 / 2.5f32) as u32; //To account for the text aspect ratio

    let loaded_img = match image::open(dir) {
        Ok(val) => val,
        Err(err) => panic!("{}",err)
    };

    let edit_image =
        loaded_img.resize_exact(
            width,
            height,
            FilterType::Lanczos3).grayscale();

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let pixel = edit_image.get_pixel(x, y);
            //image is grayscale -> every color value represents the brightness of that pixel
            let brightness = pixel[0];

            //reversed because of black background
            let idx = chars.len() - 1 - (brightness as f32 / div) as usize;

            line.push(chars[idx]);
        }
        println!("{}",line);
    }
}

fn main() {
    let mut args = env::args().skip(1);
    get_image(
        &args.next().expect("No path specified"),
        args.next().expect("No width specified").parse().expect("Invalid width"),
        args.next().expect("No height specified").parse().expect("Invalid height"));
}
