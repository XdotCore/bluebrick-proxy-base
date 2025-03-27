use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;

#[derive(WrapperApi)]
struct BBApi {
    start_bluebrick: extern "C" fn(platform: Platform, renderer: Renderer),
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub enum Renderer {
    DX9,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub enum Platform {
    Win32,
}

pub fn load_bluebrick(platform: Platform, renderer: Renderer) {
    let bluebrick = match unsafe { Container::<BBApi>::load("bluebrick/bluebrick") } {
        Ok(bb) => bb,
        Err(e) => {
            let _ = msgbox::create("Error Loading BlueBrick", &format!("Problem opening loader:\n{e:?}"), msgbox::IconType::Error);
            return;
        }
    };

    bluebrick.start_bluebrick(platform, renderer);

    std::mem::forget(bluebrick); // keeps from dropping, which would unload bluebrick
}
