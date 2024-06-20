---
id: Qrcode
title: How to create QRCode
sidebar_label: How to create QRCode
sidebar_position: 1
draft: false
hide_title: false
hide_table_of_contents: false
keywords:
  - introduction
  - Generate
  - Rust
  - QrCode
  - Back-end
images: 
last_update:
  date: 2024-5-18
  author: Phuc Lam
slug: /
---

# Generate QRCode.

  Trong quá trình phát triển dự án, mình gặp phải một vấn đề nhỏ. Mình cần thêm một hàm mới để thực hiện chức năng mời bạn bè xem kết quả và lưu thông tin vào cơ sở dữ liệu.

  Ý tưởng là khi người dùng ấn nút "Mời", hệ thống sẽ tạo ra một mã QR Code. Khi người được mời quét mã QR Code này, thông tin mời sẽ được lưu trữ vào cơ sở dữ liệu.

  Bài viết này sẽ hướng dẫn cách tạo mã QR Code mời bạn bè trong dự án..
# Những Thư Viện Cần Để Có.

- **qrcode_generator**
- **image**
- **Rust**


>[!NOTE]
>Đây là một bài viết *chia sẽ kinh nghiệm cá nhân* trong quá trình học hỏi, hi vong được mọi người góp ý nếu có lỗi.
# Demo
```rust 
use image::RgbImage;  
use qrcode_generator::QrCodeEcc;  
fn main() {  
	let link = "your_link";  
	let result: Vec<Vec<bool>> =  
	qrcode_generator::to_matrix(link, QrCodeEcc::High).unwrap();  
    
	let qr_width = result.len();  
	let qr_height = result[0].len();  
  
	let width = qr_width + 4;  
	let height = qr_height + 4;  
  
	let mut img = RgbImage::new(width as u32, height as u32);  
    
	for x in 0..width {  
		img.put_pixel(x as u32, 0, image::Rgb([255, 255, 255])); 
		img.put_pixel(x as u32, height as u32 - 1, image::Rgb([255, 255, 255]));  
	}  
	for y in 0..height {  
		img.put_pixel(0, y as u32, image::Rgb([255, 255, 255]));  
		img.put_pixel(width as u32 - 1, y as u32, image::Rgb([255, 255, 255]));
	}  
  
	let center_x = (width / 2) - (qr_width / 2);  
	let center_y = (height / 2) - (qr_height / 2);  
    
	for (y, row) in result.iter().enumerate() {  
		for (x, pixel) in row.iter().enumerate() {  
			if *pixel {  
				img.put_pixel(  
					(x + center_x) as u32,  
					(y + center_y) as u32,  
					image::Rgb([0, 0, 0]),  
				);  
			} else {  
				img.put_pixel(  
					(x + center_x) as u32,  
					(y + center_y) as u32,  
					image::Rgb([255, 255, 255]),  
				);
			}  
		}  
	}  
   
	img.save("qrcode.png").unwrap();  
}
```

- Đầu tiên mình sẽ tạo 1 biến để gắng link vào , ví dụ của mình link google
##### Khởi Tạo QR-code

- Tạo mã QR với biến link , dữ liệu sẽ trả về dưới dạng ma trận 2 chiều , mỗi phần tử trong ma trận là 1 giá trị Boolean
```rust
let result: Vec<Vec<bool>> =  
	qrcode_generator::to_matrix(link, QrCodeEcc::High).unwrap();  
```

   => **result:**:  Đây là khai báo biến `result` để lưu trữ dữ liệu mã QR. Kiểu dữ liệu là `Vec<Vec<bool>>`, đại diện cho một ma trận 2 chiều. Mỗi phần tử trong ma trận này sẽ là một giá trị Boolean (`true` hoặc `false`) biểu thị màu đen hoặc trắng của pixel trong mã QR.
   
   =>**qrcode_generator::to_matrix**: Hàm này được định nghĩa trong thư viện `qrcode_generator`. Nó nhận hai tham số:
	- **link**: Chuỗi URL (link) mà bạn muốn tạo mã QR từ.
	- **QrCodeEcc::High**: Đây là tham số xác định mức độ sửa lỗi (Error Correction Level) của mã QR. QrCodeEcc::High là mức độ cao nhất, tức là mã QR sẽ có khả năng sửa lỗi tốt nhất

  =>**QrCodeEcc** là một kiểu enum trong thư viện qrcode_generator của Rust. Nó đại diện cho **mức độ sửa lỗi (Error Correction Level)** của mã QR.
 .=>**QrCodeEcc** có `4` mức độ sửa lỗi:

 1. **Low :** Mức độ sửa lỗi thấp nhất. Sửa lỗi cho tối đa 7% dữ liệu.
 2. **Medium :** Mức độ sửa lỗi trung bình. Sửa lỗi cho tối đa 15% dữ liệu.
 3. **Quartile :** Mức độ sửa lỗi khá cao. Sửa lỗi cho tối đa 25% dữ liệu
 4. **High :** Mức độ sửa lỗi cao nhất. Sửa lỗi cho tối đa 30% dữ liệu.
![[QrCodeEcc.png]]
**=> Hình ảnh minh họa mức độ sửa lỗi từ low đến High. đây là hình ảnh có cùng độ rộng ( Width ) và chiều cao ( Height ) là 65%

>[!WARNING]
>- Mức độ sửa lỗi càng cao thì kích thước mã QR càng lớn.
>- Mức độ sửa lỗi cao hơn phù hợp cho các ứng dụng cần độ tin cậy cao, như khi mã QR bị che khuất một phần hoặc bị nhiễu.

##### **Xác Định Kích Thước và Tạo QR :**
``` rust
	let qr_width = result.len();  
	let qr_height = result[0].len();  
  
	let width = qr_width + 4;  
	let height = qr_height + 4;  
```

**- Xác định kích thước mã QR:**

* `let qr_width = result.len();`: Lấy chiều rộng của mã QR bằng cách đếm số hàng trong ma trận `result`.
* `let qr_height = result[0].len();`: Lấy chiều cao của mã QR bằng cách đếm số cột trong hàng đầu tiên của ma trận `result`.

**- Tạo kích thước hình ảnh:**

* `let width = qr_width + 4;`:  Tính toán chiều rộng của hình ảnh bằng cách cộng thêm 4 pixel vào chiều rộng của mã QR (cho viền).
* `let height = qr_height + 4;`: Tính toán chiều cao của hình ảnh bằng cách cộng thêm 4 pixel vào chiều cao của mã QR (cho viền).
##### Tạo Ảnh QR

```rust
let mut img = RgbImage::new(width as u32, height as u32);  
```
**=> Đoạn Code Tạo HÌnh Ảnh**

Tạo hàm tạo ảnh sữ dụng hàm RgImage::new(). Đây là một hàm constructor (hàm khởi tạo) của kiểu dữ liệu RgbImage, được định nghĩa trong thư viện image

   => **RgbImage** là một kiểu dữ liệu trong thư viện image của Rust, đại diện cho một hình ảnh RGB (Red, Green, Blue). Hình ảnh RGB là loại hình ảnh phổ biến nhất, sử dụng ba kênh màu đỏ, xanh lá cây và xanh dương để tạo ra màu sắc.
   => **Đặc điểm của RgbImage:**

- **Ma trận pixel:** RgbImage lưu trữ dữ liệu hình ảnh dưới dạng một ma trận hai chiều (2D) của các pixel. Mỗi pixel được biểu diễn bằng ba giá trị byte, đại diện cho cường độ màu đỏ, xanh lá cây và xanh dương.
\
##### Tạo Viền Trắng

```rust
 	for x in 0..width {  
		img.put_pixel(x as u32, 0, image::Rgb([255, 255, 255])); 
		img.put_pixel(x as u32, height as u32 - 1, image::Rgb([255, 255, 255]));  
	}  
	for y in 0..height {  
		img.put_pixel(0, y as u32, image::Rgb([255, 255, 255]));  
		img.put_pixel(width as u32 - 1, y as u32, image::Rgb([255, 255, 255]));
	}  
```
**=> Đoạn code này vẽ viền trắng xung quanh hình ảnh** 

- Hai vòng lặp for này lặp qua tất cả các pixel trên cạnh của hình ảnh:
    
    - Vòng lặp for x duyệt qua tất cả các cột (x từ 0 đến width - 1) và vẽ pixel trắng ở hàng đầu tiên (y = 0) và hàng cuối cùng (y = height - 1).
    
    - Vòng lặp for y duyệt qua tất cả các hàng (y từ 0 đến height - 1) và vẽ pixel trắng ở cột đầu tiên (x = 0) và cột cuối cùng (x = width - 1).
        
- img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));: Hàm put_pixel() được sử dụng để vẽ một pixel tại tọa độ (x, y) với màu được chỉ định. image::Rgb([255, 255, 255]) tạo ra màu trắng trong không gian màu RGB.

#####  Tạo QRCode

```rust
for (y, row) in result.iter().enumerate() {  
		for (x, pixel) in row.iter().enumerate() {  
			if *pixel {  
				img.put_pixel(  
					(x + center_x) as u32,  
					(y + center_y) as u32,  
					image::Rgb([0, 0, 0]),  
				);  
			} else {  
				img.put_pixel(  
					(x + center_x) as u32,  
					(y + center_y) as u32,  
					image::Rgb([255, 255, 255]),  
				);
			}  
		}  
	}  
```
**=> Đoạn code này duyệt qua từng pixel trong ma trận result và vẽ lên hình ảnh img với màu đen hoặc trắng tương ứng.**

Nó duyệt qua từng pixel trong ma trận result:

- Nếu pixel có giá trị **true**, vẽ một pixel ***màu đen*** lên hình ảnh.
- Nếu pixel có giá trị **false**, vẽ một pixel ***màu trắng*** lên hình ảnh.

Tọa độ của pixel được tính toán bằng cách cộng thêm center_x và center_y để đảm bảo mã QR được vẽ ở **vị trí trung tâm** của hình ảnh.

##### Lưu Ảnh

```rust
img.save("qrcode.png").unwrap();  
```

   img.save("qrcode.png"): Phương thức save() được gọi trên đối tượng img (kiểu RgbImage) để lưu hình ảnh vào một tệp tin. Tham số "qrcode.png" là tên tệp tin mà bạn muốn lưu hình ảnh vào.
   
![[Path Save Image.png]]
# Tổng Kết

**=> đây là một bài viết nhỏ để hướng dẫn cách tạo 1 QRCode mình sẽ phát triển hơn các hàm có thể tạo ra từ ID hoặc Username người dùng để bạn có thể mời bạn của mình bào ứng dụng hoặc các ý định tường tự
# Git
[Link Git](https://github.com/PhucLam202/QRCode-example)
# Thông Tin Liên Hệ 

Email : lamthanhphucit@gmail.com
Git :[Link git example tương tự](https://github.com/PhucLam202)
Facebook: [Rust Developers Vietnam](https://www.facebook.com/groups/rustdevelopersvietnam/)