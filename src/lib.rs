use ghe2d::{image::codecs::png::{CompressionType, FilterType}, raqote::SolidSource, Ghe2d, font::LoadFont};
use std::{ffi::c_void, slice};
use std::os::raw::c_char;

#[repr(C)]
pub struct Buffer {
    data: *const u8,
    len: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

#[no_mangle]
pub extern "C" fn create_img(w: i32, h: i32) -> *mut c_void {
    let img = ghe2d::Ghe2d::new(w, h);

    // std::mem::forget(d);
    Box::into_raw(Box::new(img)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn draw_circle(mut ptr: *mut c_void, x: f32, y: f32, r: f32, s: f32, e: f32, rgba: RGBA) {
    let mut img = unsafe {
        Box::from_raw(ptr as *mut Ghe2d)
    };

    img.draw_circle(x, y, r, s, e,
        ghe2d::raqote::Source::Solid(SolidSource::from_unpremultiplied_argb(rgba.alpha, rgba.red, rgba.green, rgba.blue))
    );

    ptr = Box::into_raw(Box::new(img)) as *mut c_void;
    _ = ptr;
}

#[no_mangle]
pub extern "C" fn draw_rect(mut ptr: *mut c_void, x: f32, y: f32, w: f32, h: f32, rgba: RGBA) {
    let mut img = unsafe {
        Box::from_raw(ptr as *mut Ghe2d)
    };

    img.draw_rect(x, y, w, h,
        ghe2d::raqote::Source::Solid(SolidSource::from_unpremultiplied_argb(rgba.alpha, rgba.red, rgba.green, rgba.blue))
    );

    ptr = Box::into_raw(Box::new(img)) as *mut c_void;
    _ = ptr;
}

#[no_mangle]
pub extern "C" fn draw_text(mut ptr: *mut c_void, mut load_font: *mut c_void, text: *const c_char, len: usize, x: f32, y: f32, s: f32, rgba: RGBA) {
    let mut img = unsafe {
        Box::from_raw(ptr as *mut Ghe2d)
    };

    let font = unsafe {
        Box::from_raw(load_font as *mut LoadFont)
    };

    img.draw_text(*font.clone(), get_c_str(text, len), x, y, s,
        ghe2d::utility::Rgba { r: rgba.red, g: rgba.green, b: rgba.blue, a: rgba.alpha }
    );

    ptr = Box::into_raw(Box::new(img)) as *mut c_void;
    load_font = Box::into_raw(Box::new(font)) as *mut c_void;
    _ = ptr;
    _ = load_font;
}

#[no_mangle]
pub extern "C" fn load_font(path: *const c_char, len: usize) -> *mut c_void {
    Box::into_raw(Box::new(LoadFont::new(get_c_str(path, len)))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn load_image(mut ptr: *mut c_void, path: *const c_char, len: usize, x: f32, y: f32, w: f32, h: f32, is_circle: bool) {
    let mut img = unsafe {
        Box::from_raw(ptr as *mut Ghe2d)
    };

    img.load_image(get_c_str(path, len).as_str(), x, y, w, h, is_circle);

    ptr = Box::into_raw(Box::new(img)) as *mut c_void;
    _ = ptr;
}

#[no_mangle]
pub extern "C" fn buffer(mut ptr: *mut c_void) -> Buffer{
    let img = unsafe {
        Box::from_raw(ptr as *mut Ghe2d)
    };

    let d = img.get_png_buffer(CompressionType::Fast, FilterType::Sub);

    let raw_ptr = d.as_ptr();
    let len = d.len();

    std::mem::forget(d);

    ptr = Box::into_raw(Box::new(img)) as *mut c_void;
    _ = ptr;

    Buffer{
        data: raw_ptr,
        len
    }
}

#[no_mangle]
pub extern "C" fn save(mut ptr: *mut c_void, path: *const c_char, len: usize) {
    let img = unsafe {
        Box::from_raw(ptr as *mut Ghe2d)
    };

    img.save(get_c_str(path, len).as_str()).unwrap();

    ptr = Box::into_raw(Box::new(img)) as *mut c_void;
    _ = ptr;
}

fn get_c_str(ptr: *const c_char, len: usize) -> String {
    let slice = unsafe {
        slice::from_raw_parts(ptr as *const u8, len)
    };
    return String::from_utf8(slice.to_vec()).unwrap();
}
