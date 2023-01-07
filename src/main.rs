use qrcode::render::unicode;
use qrcode::QrCode;
use std::io::BufRead;

fn main() {
    let lines = std::io::stdin().lock().lines();

    for line in lines {
        let last_input = line.unwrap();

        let code = QrCode::new(last_input).unwrap();
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{}", image);
    }
}
