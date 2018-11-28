use melon::{typedef::*, System, VM};
use minifb::{Scale, Window, WindowOptions};
use rand::{distributions::Standard, prelude::*};

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 128;

pub struct MatrixSystem {
    buffer: Vec<u32>,
    window: Window,
    color_map: Vec<u32>,
}

impl MatrixSystem {
    pub fn new() -> MatrixSystem {
        MatrixSystem {
            buffer: gen_random(),
            window: Window::new(
                "tmod",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WindowOptions {
                    scale: Scale::X4,
                    ..Default::default()
                },
            )
            .unwrap(),
            color_map: (0..=255)
                .map(|color_byte: u8| {
                    let red: u8 = ((color_byte >> 5) & 0b111) * 36;
                    let green: u8 = ((color_byte >> 2) & 0b111) * 36;
                    let blue: u8 = (color_byte & 0b11) * 85;

                    let mut color = (red as u32) << 16;
                    color |= (green as u32) << 8;
                    color |= blue as u32;

                    color
                })
                .collect(),
        }
    }

    fn write_current_row_to_buffer(&mut self, vm: &mut VM) -> Result<()> {
        let row = vm.pop_u8()?;

        self.write_row(row as usize, &vm.mem[..SCREEN_WIDTH])
    }

    fn write_row(&mut self, row: usize, buf: &[u8]) -> Result<()> {
        assert!(row < SCREEN_HEIGHT, "Row exceeds screen height");

        let row_start = row * SCREEN_WIDTH;
        let owned_colors: Vec<_> = buf
            .iter()
            .map(|byte| self.color_map[*byte as usize])
            .collect();

        self.buffer
            .iter_mut()
            .skip(row_start)
            .take(SCREEN_WIDTH)
            .enumerate()
            .for_each(|(idx, color)| *color = owned_colors[idx]);

        Ok(())
    }
}

impl System for MatrixSystem {
    const ID: &'static str = "org.bakervm.tmod";

    const MEM_PAGES: u8 = 1;

    fn system_call(&mut self, vm: &mut VM, signal: u16) -> Result<()> {
        match signal {
            1 => self.window.update_with_buffer(&self.buffer[..]).unwrap(),
            2 => self.write_current_row_to_buffer(vm)?,
            _ => {}
        }

        Ok(())
    }
}

fn gen_random() -> Vec<u32> {
    thread_rng()
        .sample_iter(&Standard)
        .take(SCREEN_WIDTH * SCREEN_HEIGHT)
        .collect()
}
