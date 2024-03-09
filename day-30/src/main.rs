const WIDTH: i32 = 100; // eventually

fn main() {
    let mut draw_pixel = false;
    let mut x_index = 0;
    for pixels_str in include_str!("input.txt").split_ascii_whitespace() {
        let mut num_pixels: i32 = str::parse(pixels_str).unwrap();

        while num_pixels > 0 {
            print!("{}", if draw_pixel { "#" } else { "." });
            num_pixels -= 1;
            x_index += 1;
            if x_index == WIDTH {
                println!();
                x_index = 0;
            }
        }
        draw_pixel = !draw_pixel;
    }
    println!()
}
