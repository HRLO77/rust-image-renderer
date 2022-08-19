use image;
use image::GenericImageView;

fn main(){
    let img: image::DynamicImage = image::open("256px-wallpaper1.webp").expect("File not found!");
    let width: u32 = img.width();
    let mut s: String = String::new();
    for pixel in img.pixels() {
        let f = &(pixel.2[0].to_string()+";"+&pixel.2[1].to_string()+";"+&pixel.2[2].to_string()) as &str;
        if pixel.0 == width-1{
            s.push_str(&("\x1b[38;2;".to_owned()+f+"m███\x1b[0m\n") as &str)
        }
        else{
            s.push_str(&("\x1b[38;2;".to_owned()+f+"m███\x1b[0m") as &str)
        }
    }
    println!("{}", s);
}