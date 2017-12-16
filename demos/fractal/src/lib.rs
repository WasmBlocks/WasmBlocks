#[macro_use(wasmblock_setup)]
extern crate wasmblock;

//using special macros for global state, see below
use std::mem;
use std::os::raw::{c_char,c_void};
use std::ffi::{CString};
use wasmblock::{dom,canvas};

// needed for allocation and deallocation functions
wasmblock_setup!();

#[no_mangle]
pub fn start() -> () {
    dom::create_element("body","style","game_styles");
    dom::set_inner_html("#game_styles",include_str!("fractal.css"));
    dom::create_element("body","canvas","screen");
    dom::set_attribute("#screen","width","600");
    dom::set_attribute("#screen","height","400");
    let ctx = canvas::get_context("#screen");
    for x in 0..600 {
        for y in 0..400 {
            let r = ((x as f32)/600.0 * 255.0) as u8;
            let g = ((y as f32)/600.0 * 255.0) as u8;
            let b = 0;
            let a = 1.0;
            canvas::set_fill_style_color(ctx,r,g,b,a);
            canvas::fill_rect(ctx,x as f32,y as f32,1.0,1.0);
        }
    }
}
