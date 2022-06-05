use std::fs::File;
extern crate image;
use image::{ImageBuffer, RgbImage};

fn main() {
    println!("Hello, world!");

    let bytes = std::fs::read("pi_hex_1b.txt").unwrap();
    // amogus pattern
    let pattern = "7cf5".as_bytes();

    let mut results: Vec<usize> = Vec::new();

    for i in 0..bytes.len()-(pattern.len()-1) {
        let mut found=true;
        for y in 0..pattern.len() {
            if !(bytes[i+y]==pattern[y]){
                found=false;
                break;
            }
        }
        if found {
            println!("found at offset {}",i*4);
            results.push(i);
            buildimg(i, &bytes)
        }
    }
}



fn buildimg(offset:usize, buffer: &Vec<u8>) {

    let table = [('0',"0000"),('1',"0001"),('2',"0010"),('3',"0011"),('4',"0100"),('5',"0101"),('6',"0110"),('7',"0111"),('8',"1000"),('9',"1001"),('a',"1010"),('b',"1011"),('c',"1100"),('d',"1101"),('e',"1110"),('f',"1111")];

    let length:u32 = 16;
    let width=4;
    let mut image: RgbImage = ImageBuffer::new(width, length);

    let mut bitoffset=(offset-(length as usize/2))*4;

    for y in 0..length {
        for x in 0..width {
            let mut chart=0;
            for t in 0..16 {
                if buffer[bitoffset/4 as usize]==table[t].0 as u8{
                    chart=table[t].1.as_bytes()[bitoffset%4];
                    break;
                }
            }

            match chart as char {
                '0' => {*image.get_pixel_mut(x, y)=image::Rgb([0,0,0]);}
                '1' => {*image.get_pixel_mut(x, y)=image::Rgb([255,255,255]);}
                _ => {}
            }

            bitoffset+=1;
        }
    }

    image.save(format!("img/pi_{}.png",offset*4)).unwrap();
}