extern crate image;

use std::env;

fn main() {
    // 引数解析
    let mut arg = env::args();
    // 実行コマンドをスキップ
    arg.next();

    // 引数チェック
    let image_path = match arg.next() {
        None => {
            println!("error");
            return
        },
        Some(s) => s,
    };

    //引数を得られているかデバック
    println!("Image Path is {}",image_path);
    test(image_path);
}

fn test(path : String){
    // ファイル展開とエラーチェック(.unwrap())
    let     img      = image::open(path).unwrap();
    let     img      = img.to_rgb8();
    let     size_x   = img.width();
    let     size_y   = img.height();

    // グレースケール用のデータを作成
    let mut gray_img = image::GrayImage::new(size_x,size_y);
    let pix  = img.get_pixel(0,0);

    for y in 0 .. size_y {
        for x in 0 .. size_x {
            // ピクセルデータの取得
            let pix  = img.get_pixel(x,y);
            // GrayScale BT.709
            // V = 0.2126*R + 0.7152*G + 0.0722*B
            let val  = [((pix[0] as f32 * 0.2126) as u32 +
                         (pix[1] as f32 * 0.7152) as u32 +
                         (pix[2] as f32 * 0.0722) as u32) as u8;1];
            // ピクセルデータをあらかじめ作っておいたグレースケールデータに書き込む
            gray_img.put_pixel(x,y,image::Luma(val));
        }
    }

    // 画像をファイルとして保存する。エラーチェックも忘れずに
    gray_img.save("./gray.png").unwrap();
}
