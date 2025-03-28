use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;

#[derive(WrapperApi)]
struct BBApi {
    start_bluebrick: extern "C" fn(platform: RequestedPlatform, renderer: RequestedRenderer),
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub enum RequestedRenderer {
    DX9,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub enum RequestedPlatform {
    Win32,
}

pub fn load_bluebrick(platform: RequestedPlatform, renderer: RequestedRenderer) {
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
