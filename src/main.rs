use image::RgbImage;
use qrcode_generator::QrCodeEcc;
fn main() {
    let link = "https://www.facebook.com/share/p/wNzSPgM8vhYa5Vct/";
    let result: Vec<Vec<bool>> =
        qrcode_generator::to_matrix(link, QrCodeEcc::Low).unwrap();
        println!("result :{:?}",result);
    // Tạo hình ảnh
    let qr_width = result.len();
    let qr_height = result[0].len();
    println!("qr_width :{}",qr_width);
    println!("qr_height :{}",qr_height);

    let width = qr_width + 4;
    let height = qr_height + 4;

    let mut img = RgbImage::new(width as u32, height as u32);

    // Vẽ viền xung quanh hình ảnh
    for x in 0..width {
        img.put_pixel(x as u32, 0, image::Rgb([255, 255, 255])); // Hàng trên
        img.put_pixel(x as u32, height as u32 - 1, image::Rgb([255, 255, 255]));
        // Hàng dưới
    }
    for y in 0..height {
        img.put_pixel(0, y as u32, image::Rgb([255, 255, 255])); // Cột trái
        img.put_pixel(width as u32 - 1, y as u32, image::Rgb([255, 255, 255])); // Cột phải
    }

    // Tính toán vị trí trung tâm để vẽ mã QR
    let center_x = (width / 2) - (qr_width / 2);
    let center_y = (height / 2) - (qr_height / 2);

    // Vẽ mã QR ở vị trí trung tâm
    for (y, row) in result.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            if *pixel {
                img.put_pixel(
                    (x + center_x) as u32,
                    (y + center_y) as u32,
                    image::Rgb([0, 0, 0]),
                ); // Màu đen
            } else {
                img.put_pixel(
                    (x + center_x) as u32,
                    (y + center_y) as u32,
                    image::Rgb([255, 255, 255]),
                ); // Màu trắng
            }
        }
    }

    // Lưu hình ảnh
    img.save("qrcode.png").unwrap();
}
