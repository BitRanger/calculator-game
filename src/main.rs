#![no_std]
#![no_main]

use core::cmp::Ordering;

pub mod eadk;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"HelloRust\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

pub mod game {
    use crate::eadk;
    #[derive(Debug, Clone, Copy)]
    pub struct Player {
        pub rect: eadk::Rect,
        pub color: eadk::Color,
        pub movement: [i16; 2],
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Keys {
        pub up: bool,
        pub down: bool,
        pub left: bool,
        pub right: bool,
        pub home: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Pipe {
        pub top: eadk::Rect,
        pub bottom: eadk::Rect,
        pub color: eadk::Color,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Background {
        pub rect: eadk::Rect,
        pub color: eadk::Color,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Game {
        pub player: Player,
        pub pipes: [Pipe; 3],
        pub keys: Keys,
        pub last_keys: Keys,
        pub background: Background,
        pub keyboard: eadk::input::KeyboardState,
    }

    #[allow(dead_code)]
    fn random_u16() -> u16 {
        crate::eadk::random() as u16
    }

    #[allow(dead_code)]
    fn random_coordinate() -> u16 {
        (crate::eadk::random() % 0xFF) as u16
    }
}

#[no_mangle]
fn main() {
    let mut game = game::Game {
        player: game::Player {
            rect: eadk::Rect {
                x: 60,
                y: 120,
                width: 20,
                height: 20,
            },
            color: eadk::Color { rgb565: 31 }, // blue
            movement: [0, 0],
        },

        keys: game::Keys {
            up: false,
            down: false,
            left: false,
            right: false,
            home: false,
        },
        last_keys: game::Keys {
            up: false,
            down: false,
            left: false,
            right: false,
            home: false,
        },
        keyboard: eadk::input::KeyboardState::scan(),
    };

    loop {
        /* SCAN KEYBOARD */
        game.keyboard = eadk::input::KeyboardState::scan();

        game.last_keys = game.keys;

        /* INPUT CHECK */
        game.keys.left =
            eadk::input::KeyboardState::key_down(&game.keyboard, eadk::input::Key::Left);
        game.keys.right =
            eadk::input::KeyboardState::key_down(&game.keyboard, eadk::input::Key::Right);
        game.keys.up = eadk::input::KeyboardState::key_down(&game.keyboard, eadk::input::Key::Up);
        game.keys.down =
            eadk::input::KeyboardState::key_down(&game.keyboard, eadk::input::Key::Down);
        game.keys.home =
            eadk::input::KeyboardState::key_down(&game.keyboard, eadk::input::Key::Home);

        /* MOVEMENT */

        // vertical
        if game.keys.up && !game.last_keys.up {
            game.player.movement[1] = -5;
        }

        match game.player.movement[1].cmp(&0_i16) {
            Ordering::Less => {
                game.player.movement[1] = -game.player.movement[1];
                if game.player.rect.y as i16 - game.player.movement[1] < 0 {
                    game.player.rect.y = 0;
                    game.player.movement[1] = 0;
                } else {
                    game.player.rect.y -= game.player.movement[1] as u16;
                    game.player.movement[1] = -game.player.movement[1];
                    game.player.movement[1] += 1;
                }
            }
            Ordering::Greater => {
                if game.player.rect.y + game.player.movement[1] as u16 >= 220 {
                    game.player.rect.x = 60;
                    game.player.rect.y = 120;
                    game.player.movement[1] = 0;
                } else {
                    game.player.rect.y += game.player.movement[1] as u16;
                    game.player.movement[1] += 1;
                }
            }
            Ordering::Equal => {
                if game.player.rect.y < 220 {
                    game.player.movement[1] += 1;
                }
            }
        }

        /* RENDERING */
        eadk::display::wait_for_vblank();

        // background
        eadk::display::push_rect_uniform(
            eadk::Rect {
                x: 0,
                y: 0,
                width: 320,
                height: 240,
            },
            eadk::Color { rgb565: 65535 },
        );

        // player
        eadk::display::push_rect_uniform(game.player.rect, game.player.color);
    }
}

/*
 * Initial Application Test:
 */

/*
#[no_mangle]
pub fn main() {
    let white = eadk::Color { rgb565: 65535 };
    let black = eadk::Color { rgb565: 0 };
    let red = eadk::Color { rgb565: 63488 };
    let green = eadk::Color { rgb565: 2016 };
    let blue = eadk::Color { rgb565: 31 };

    let background = eadk::Rect {
        x: 0,
        y: 0,
        width: 320,
        height: 240,
    };

    loop {
        let keyboard = eadk::input::KeyboardState::scan();
        eadk::display::wait_for_vblank();
        if eadk::input::KeyboardState::key_down(&keyboard, eadk::input::Key::One) == true {
            eadk::display::push_rect_uniform(background, black);
        } else if eadk::input::KeyboardState::key_down(&keyboard, eadk::input::Key::Two) {
            eadk::display::push_rect_uniform(background, red);
        } else if eadk::input::KeyboardState::key_down(&keyboard, eadk::input::Key::Three) == true {
            eadk::display::push_rect_uniform(background, green);
        } else if eadk::input::KeyboardState::key_down(&keyboard, eadk::input::Key::Four) {
            eadk::display::push_rect_uniform(background, blue);
        } else {
            eadk::display::push_rect_uniform(background, white);
        }
    }
}
*/
