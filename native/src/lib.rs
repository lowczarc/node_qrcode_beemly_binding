#[macro_use]
extern crate neon;
extern crate beautiful_qrcode_generator_beemly;
extern crate image;

use std::io::Cursor;
use std::ops::DerefMut;

use image::{ImageOutputFormat, DynamicImage};

use beautiful_qrcode_generator_beemly::qrcode_generate;
use neon::prelude::*;

fn generate_qrcode(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let mut buff = Cursor::new(Vec::new());
    let message = cx.argument::<JsString>(0)?.value();

    DynamicImage::ImageRgb8(qrcode_generate(&message)).write_to(&mut buff, ImageOutputFormat::PNG).unwrap();

    let vec = buff.into_inner();
    let mut js_buffer = JsBuffer::new(&mut cx, vec.len() as u32)?;

    cx.borrow_mut(&mut js_buffer, |mut slice| {
        let raw = slice.deref_mut().as_mut_slice();
        for (i, obj) in vec.iter().enumerate() {
            raw[i] = *obj;
        }
    });
    

    Ok(js_buffer)
}

register_module!(mut cx, {
    cx.export_function("generateQRCode", generate_qrcode)
});
