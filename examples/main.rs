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

    let img = TGAImage::from_tga_file("out.tga");
    println!(
        "{} by {}, color of (0,0) : {:?}, (0, h): {:?}, (w, 0): {:?}, (w, h): {:?}",
        img.width(),
        img.height(),
        img.get(0, 0),
        img.get(0, img.height() - 1),
        img.get(img.width() - 1, 0),
        img.get(img.width() - 1, img.height() - 1),
    );
    img.write_tga_file("out_non_rle.tga", false);
}
