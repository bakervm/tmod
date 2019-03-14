use melon::{typedef::*, System, VM};
use minifb::{Key, Scale, Window, WindowOptions};
use rand::{distributions::Standard, prelude::*};

const VIEWPORT_HEIGHT: usize = 128;
const VIEWPORT_WIDTH: usize = VIEWPORT_HEIGHT + (VIEWPORT_HEIGHT / 2);

/// The width of the background (in number of tiles)
const BACKGROUND_WIDTH: usize = (VIEWPORT_WIDTH / TILE_SIZE) * 2;
/// The height of the background (in number of tiles)
const BACKGROUND_HEIGHT: usize = (VIEWPORT_HEIGHT / TILE_SIZE) * 2;

const TILE_SIZE: usize = 8;

const PALETTE_COLOR_COUNT: usize = 16;
const PALETTE_COUNT: usize = 16;

const MAX_TILE_COUNT: usize = BACKGROUND_WIDTH * BACKGROUND_HEIGHT * 4;

const SPRITE_COUNT: usize = 256;

#[derive(Clone, Copy, Default)]
/// A tile definition. A tile consists of 8x8 pixels.
/// Each pixel is a 4-bit index of one of the 16 colors of the active sprite palette.
/// This is why this definition uses a line by line definition using 32 bit integer values:
/// ```text
///   0    1    2    3    4    5    6    7
/// 0 0000 0000 0000 0000 0000 0000 0000 0000
/// 1 0000 0000 0000 0000 0000 0000 0000 0000
/// 2 0000 0000 0000 0000 0000 0000 0000 0000
/// 3 0000 0000 0000 0000 0000 0000 0000 0000
/// 4 0000 0000 0000 0000 0000 0000 0000 0000
/// 5 0000 0000 0000 0000 0000 0000 0000 0000
/// 6 0000 0000 0000 0000 0000 0000 0000 0000
/// 7 0000 0000 0000 0000 0000 0000 0000 0000
/// ```
pub struct Tile([u32; TILE_SIZE]);

#[derive(Clone, Copy, Default)]
/// A sprite composed from multiple tiles. A sprite defines it's overall used color palette
pub struct Sprite {
    palette: u8,
    width: u8,
    height: u8,
    tiles: [u16; 32],
}

pub struct MatrixSystem {
    buffer: [u32; VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
    window: Window,
    palettes: [[u8; PALETTE_COLOR_COUNT]; PALETTE_COUNT],
    tiles: [Tile; MAX_TILE_COUNT],
    sprites: [Sprite; SPRITE_COUNT],
    /// The position of the viewport
    viewport_pos: (usize, usize),
    /// The background is actually 4 times the size of the window
    background: [Tile; BACKGROUND_WIDTH * BACKGROUND_HEIGHT],
    background_palette: u8,
}

impl MatrixSystem {
    pub fn new() -> MatrixSystem {
        MatrixSystem {
            buffer: [0; VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            window: Window::new(
                "tmod",
                VIEWPORT_WIDTH,
                VIEWPORT_HEIGHT,
                WindowOptions {
                    scale: Scale::X4,
                    ..Default::default()
                },
            )
            .unwrap(),
            palettes: [[0; PALETTE_COLOR_COUNT]; PALETTE_COUNT],
            tiles: [Default::default(); MAX_TILE_COUNT],
            sprites: [Default::default(); SPRITE_COUNT],
            viewport_pos: Default::default(),
            background: [Default::default(); BACKGROUND_WIDTH * BACKGROUND_HEIGHT],
            background_palette: 0,
        }
    }

    pub fn write_row(&mut self, row: usize, data: &[u8]) {
        let start_pos = row * VIEWPORT_WIDTH;
        let end_pos = start_pos + VIEWPORT_WIDTH;

        for (count, idx) in (start_pos..end_pos).enumerate() {
            self.buffer[idx] = u32::from(data[count]) * 2000;
        }
    }

    fn write_current_row_to_buffer(&mut self, vm: &mut VM) -> Result<()> {
        let row = vm.pop_u8()?;

        self.write_row(row as usize, &vm.mem[..VIEWPORT_WIDTH]);

        Ok(())
    }
}

impl System for MatrixSystem {
    const ID: &'static str = "org.bakervm.tmod";

    const MEM_PAGES: u8 = 1;

    fn post_cycle(&mut self, vm: &mut VM) -> Result<()> {
        if self.window.is_key_down(Key::Escape) {
            vm.halt();
        }

        Ok(())
    }

    fn system_call(&mut self, vm: &mut VM, signal: u16) -> Result<()> {
        match signal {
            1 => self.window.update_with_buffer(&self.buffer)?,
            2 => self.write_current_row_to_buffer(vm)?,
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn viewport_height_sanity() {
        assert!(VIEWPORT_HEIGHT.is_power_of_two());
    }

    #[test]
    fn tile_size_sanity() {
        assert!(TILE_SIZE.is_power_of_two());

        assert_eq!(VIEWPORT_HEIGHT % TILE_SIZE, 0);
        assert_eq!(VIEWPORT_WIDTH % TILE_SIZE, 0);
    }
}
