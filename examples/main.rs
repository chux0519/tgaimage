use tgaimage::*;

fn main() {
    let mut image = TGAImage::new(100, 100, 3);
    let red = TGAColor::rgb(255, 0, 0);
    for i in 0..image.width() {
        for j in 0..image.height() {
            image.set(i, j, &red);
        }
    }

    image.write_tga_file("out.tga");
}
