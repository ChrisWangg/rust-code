use bmp::{Image, Pixel};
use core::f32;
use std::io;

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "diagonal" { 
        diagonal(path.as_str());
    } else if operation.as_str() == "cross" {
        cross(path.as_str());
    } else if operation.as_str() == "house" {
        house(path.as_str());
    } else if operation.as_str() == "outline_s" {
        outline_square(path.as_str());
    } else if operation.as_str() == "rainbow" {
        rainbow_flag(path.as_str());
    } else if operation.as_str() == "sin" {
        sine_wave(path.as_str());
    }
    else {
        eprintln!("The operation {operation} was not recognised!");
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}
fn diagonal(path: &str) {
    let mut image = Image::new(100, 100);
    for i in 0..100 {
        for j in 0..100 {
            if i == j {
                image.set_pixel(i, j, Pixel::new(255, 255, 255));
            }
        }
    }
    image.save(path).expect("This should save correctly.");
}
fn cross(path: &str) {
    let mut image = Image::new(100, 100);
    for i in 0..100 {
        for j in 0..100 {
            if i == j {
                image.set_pixel(i, j, Pixel::new(255, 255, 255));
            } else if i + j == 100 {
                image.set_pixel(i, j, Pixel::new(255, 255, 255));
            }
        }
    }
    image.save(path).expect("This should save correctly.");
}
fn house(path: &str) {
    let mut image = Image::new(100, 100);
    for i in 40..60 {
        image.set_pixel(i, 40, Pixel::new(255, 255, 255));
        image.set_pixel(i, 60, Pixel::new(255, 255, 255));
        image.set_pixel(40, i, Pixel::new(255, 255, 255));
        image.set_pixel(60, i, Pixel::new(255, 255, 255));
    }
    
    let mut i = 40;
    let mut j = 40;
    for _ in 0..=10 {
        image.set_pixel(j, i, Pixel::new(255, 255, 255));
        j += 1;
        i -= 1;
    }
    i = 40;
    j = 60;
    for _ in 0..=10 {
        image.set_pixel(j, i, Pixel::new(255, 255, 255));
        j -= 1;
        i -= 1;
    }
    image.save(path).expect("This should save correctly.");

}

fn outline_square(path: &str) {
    let mut width = String::new();
    println!("Enter Width: ");
    io::stdin().read_line(&mut width).unwrap();
    width = width.trim().to_string();
    println!("{:?}", width);

    let width_int: u32 = width.parse().expect("int here");

    let mut coords_x = String::new();
    println!("Enter coords_x: ");
    io::stdin().read_line(&mut coords_x).unwrap();
    coords_x = coords_x.trim().to_string();
    println!("{:?}", coords_x);

    let x: u32 = coords_x.parse().expect("Please input valid int");

    let mut corrds_y = String::new();
    println!("Enter corrds_y: ");
    io::stdin().read_line(&mut corrds_y).unwrap();
    corrds_y = corrds_y.trim().to_string();
    println!("{:?}", corrds_y);
    let y: u32 = corrds_y.parse().expect("Please input valid int");

    let mut image = Image::new(100, 100);
    for i in x..x+width_int {
        image.set_pixel(i, x, Pixel::new(255, 255, 255));
        image.set_pixel(i, x + width_int, Pixel::new(255, 255, 255));
        image.set_pixel(y, i, Pixel::new(255, 255, 255));
        image.set_pixel(y + width_int, i, Pixel::new(255, 255, 255));
    }

    image.save(path).expect("This should save correctly.");
}

fn rainbow_flag(path: &str) {
    let mut image = Image::new(1000, 600);
    for i in 0..1000 {
        for j in 0..600 {
            if j < 100 {
                image.set_pixel(i, j, Pixel::new(255, 0, 0));
            } else if j < 200 {
                image.set_pixel(i, j, Pixel::new(255,140,0));
            } else if j < 300 {
                image.set_pixel(i, j, Pixel::new(255,255,0));
            } else if j < 400 {
                image.set_pixel(i, j, Pixel::new(127,255,0));
            } else if j < 500 {
                image.set_pixel(i, j, Pixel::new(0,127,255));
            } else if j < 600 {
                image.set_pixel(i, j, Pixel::new(0,0,255));
            } 
        }
    }
    image.save(path).expect("This should save correctly.");

}

fn sine_wave(path: &str) {
    let mut image = Image::new(1000, 800);
    for i in 0..1000 {
        let mut x = i as f32 / 1000.0;
        x = x * f32::consts::TAU;
        let y = (x.sin() * 300.0) as i32 + 400;
        println!("{x} {y}");
        image.set_pixel(i, (y) as u32, Pixel::new(127,255,0));
    }
    image.save(path).expect("This should save correctly.");
}
