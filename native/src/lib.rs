#[macro_use]
extern crate neon;
extern crate beautiful_qrcode_generator_beemly;
extern crate image;

use std::io::Cursor;

use image::{ImageOutputFormat, DynamicImage};

use beautiful_qrcode_generator_beemly::qrcode_generate;
use neon::prelude::*;

fn generate_qrcode(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let mut buff = Cursor::new(Vec::new());
    let message = cx.argument::<JsString>(0)?.value();

    DynamicImage::ImageRgb8(qrcode_generate(&message)).write_to(&mut buff, ImageOutputFormat::PNG).unwrap();

    let vec = buff.into_inner();
    let js_array = JsArrayBuffer::new(&mut cx, vec.len() as u32)?;

    for (i, obj) in vec.iter().enumerate() {
        let num = cx.number(*obj as f64);
        js_array.set(&mut cx, i as u32, num)?;
    }

    Ok(js_array)
}

register_module!(mut cx, {
    cx.export_function("generateQRCode", generate_qrcode)
});
