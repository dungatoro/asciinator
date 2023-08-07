use color_space::{Rgb, Hsv, FromColor};
use image::io::Reader;
use image::{ DynamicImage, GenericImageView, imageops};
use colored::Colorize;

mod args;
use args::AsciinatorArgs;
use clap::Parser;

fn read_image(path: &str, scale_factor: u32, stretch_factor: usize) -> Option<PixelImage> {
    let img: DynamicImage = 
        match Reader::open(path) {
            Ok(i) => i.decode().unwrap(),
            Err(e) => { println!("Failed to read file {e}"); return None }
        };

    let (width, height) = img.dimensions();
    let (width, height) = (width/scale_factor, height/scale_factor);
    let img = imageops::resize(&img, width, height, imageops::FilterType::Nearest);

    let mut flat_image: Vec<Hsv> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.0;
            let hsv: Hsv;
            if rgba[3] == 0 { // set transparent as black
                hsv = Hsv::new(0.0, 0.0, 0.0);
            } else {
                let rgb = Rgb::new(rgba[0] as f64, rgba[1] as f64, rgba[2] as f64);
                hsv = Hsv::from_color(&rgb);
            }
            flat_image.append(&mut vec![hsv;stretch_factor]);
        }
    }
    Some(PixelImage { flat_image, width: width*stretch_factor as u32, height })
}

trait Image2Ascii {
    fn print_ascii<F>(&self, convert: F) where F: Fn(&Hsv) -> String;
}

struct PixelImage {
    flat_image: Vec<Hsv>,
    width: u32,
    height: u32
}

impl Image2Ascii for PixelImage {
    fn print_ascii<F>(&self, convert: F) where F: Fn(&Hsv) -> String {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.flat_image[(x+y*self.width) as usize];
                let c = convert(&pixel);
                let colour = Rgb::from_color(&Hsv::new(pixel.h, pixel.s, 1.0));
                print!("{}", c.truecolor(colour.r as u8, colour.g as u8, colour.b as u8));
            }
            println!();
        }
    }
}

fn main() {
    let args = AsciinatorArgs::parse();
    let img = read_image(&args.image_path, args.scale_factor, args.stretch_factor).unwrap();

    let ramp: Vec<String> = args.ramp.chars().map(|c| c.to_string()).collect();
    let convert = |hsv: &Hsv| {
            let index = (hsv.v * (args.ramp.len()-1) as f64) as usize;
            ramp[index].clone()
    };

    img.print_ascii(convert);
    
}

