use image::{GenericImageView, imageops::FilterType};
use std::env;

fn get_image(dir: &str, width: u32, height: u32) {
    let chars = ['$','@','B','%','8','&','W','M','#','*','o','a','h','k','b','d',
    'p','q','w','m','Z','O','0','Q','L','C','J','U','Y','X','z','c','v','u','n','x','r','j','f','t','/',
        '\\','|','(',')','1','{','}','[',']','?','-','_','+','~','<','>','i','!','l','I',';',':',',','"',
        '^','`','\'','.',' '];
    let div: f32 = 255f32 / chars.len() as f32;

    let loaded_img = match image::open(dir) {
        Ok(val) => val,
        Err(err) => panic!("{}",err)
    };

    let edit_image =
        loaded_img.resize_exact(
            width,
            height/2, //To account for the text aspect ratio
            FilterType::Triangle).grayscale();

    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let pixel = edit_image.get_pixel(x, y);
            //image is grayscale -> every color value represents the brightness of that pixel
            let brightness = pixel[0];

            let idx =  (brightness as f32 / div) as usize;

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
