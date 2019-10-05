use tgaimage::*;

fn main() {
    let mut image = TGAImage::new(255, 255, 3);
    for i in 0..image.width() {
        for j in 0..image.height() {
            let color = TGAColor::rgb(i as u8, j as u8, 128);
            image.set(i, j, &color);
        }
    }

    image.write_tga_file("out.tga", true);
}
