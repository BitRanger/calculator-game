#![no_std]
#![no_main]

use core::cmp::Ordering;

pub mod eadk;

// Declare name
#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 12] = *b"Flappy Bird\0";

// Declare api level TODO: Find out what eadk api level is
#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

// Declare icon
#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

pub mod game {
    use crate::eadk;

    pub struct Player {
        pub texture: Texture,
        pub movement: [f32;2],
    }
    impl Player {
        pub fn render(&self) {
            eadk::display::push_rect(self.texture.rect, self.texture.image);
        }
        pub fn movement() {
            
        }
    }
    
    // actually a pair of pipes; top & bottom
    pub struct Pipe {
        pub top: Texture,
        pub bottom: Texture,
        pub movement: [f32;2],
    }
    impl Pipe {
        pub fn render(&self) {
            eadk::display::push_rect(self.top.texture.rect, self.top.texture.image);
            eadk::display::push_rect(self.bottom.texture.rect, self.bottom.texture.image);
        }
        pub fn movement() {
            
        }       
    }
    
    pub struct Background {
        pub texture: UniformTexture,
    }
    impl Background {
        pub fn render(&self) {
            eadk::display::push_rect(self.texture.rect, self.texture.color);
        }        
    }

    pub struct Texture {
        rect: eadk::Rect,
        image: &[eadk::Color],
    }
    pub struct UniformTexture {
        rect: eadk::Rect,
        color: eadk::Color,
    }
    pub struct Game {
        player: Player,
        pipes: [Pipe;3],
        background: Background,
    }
    impl Game {
        pub fn render() {
            self.player.render();
        }
    }

}


#[no_mangle]
fn main() {
    let mut game: game::Game = game::Game {
        player: game::Player {
            texture: game::Texture {
                rect: eadk::Rect {
                    width: 20,
                    height: 20,
                    x: 0,
                    y: 0,
                },
                image: images::
            },
            movement: [0.0, 0.0],
        },
        pipes: [
            game::Pipe {
                
            },
            game::Pipe {
                
            },
            game::Pipe {

            },
        ],
        background: game::Background {
            texture: game::UniformTexture {
                rect: eadk::Rect { x: 0, y: 0, width: 320, height:  240 },
                color: eadk::Color { rgb565: 0x469d },
            },
        },
    };
    loop {
        
    }
}
