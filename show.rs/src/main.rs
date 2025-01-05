use argh::FromArgs;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::io::prelude::*;

const MARGIN: usize = 4;
const BACKGROUND: u32 = 0x333333;
const FOREGROUND: u32 = 0xF1F11F;

#[derive(FromArgs)]
/// Demo the font
struct Config {
    /// give the path to the font
    #[argh(option, default = "String::from(\"../SpleenttMedium-8.bdf\")")]
    font: String,

    /// show the default sample
    #[argh(switch, short = 'd')]
    default: bool,

    /// if given, dump a screenshot here in PPM format
    #[argh(option)]
    ppmdump: Option<String>,

    /// show as ascii chars
    #[argh(switch, short = 'a')]
    asciidump: bool,

    /// scale pixels by 2X
    #[argh(switch, short = 's')]
    scale: bool,

    /// if given, sets the background color in hex (without # or 0x)
    #[argh(option)]
    background: Option<String>,

    /// if given, sets the foreground color in hex (without # or 0x)
    #[argh(option)]
    foreground: Option<String>,

    /// arguments
    #[argh(positional, greedy)]
    text: Vec<String>,
}

struct Context {
    window: Window,
    font: bdf::Font,
    width: usize,
    height: usize,
    scale: usize,
    cursor: (usize, usize),
    foreground: u32,
    fb: Vec<u32>,
}

impl Context {
    fn new(font: bdf::Font, width: usize, height: usize, scalex2: bool) -> anyhow::Result<Context> {
        let mut window = Window::new(
            "spleentt - hit Escape to exit",
            width,
            height,
            WindowOptions {
                resize: true,
                scale: if scalex2 { Scale::X2 } else { Scale::X1 },
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
        .expect("Unable to Open Window");

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Ok(Context {
            window,
            width,
            height,
            font,
            cursor: (MARGIN, MARGIN),
            scale: if scalex2 { 4 } else { 2 },
            foreground: FOREGROUND,
            fb: vec![BACKGROUND; width * height],
        })
    }

    fn set_foreground(&mut self, foreground: u32) {
        self.foreground = foreground;
    }

    fn set_background(&mut self, background: u32) {
        self.fb = vec![background; self.width * self.height];
    }

    fn renderline(&mut self, s: &str) {
        let default = self.font.glyphs().get(&'?').unwrap();

        for y in 0..8 {
            for c in s.chars() {
                let glyph = self.font.glyphs().get(&c).unwrap_or(default);
                let bb = glyph.bounds();
                let (height, width) = (bb.height as i32, bb.width as i32); // Sigh

                for x in 0..5 {
                    let gy = y + bb.y - (8 - height);
                    let gx = x - bb.x;

                    if (0..height).contains(&gy)
                        && (0..width).contains(&gx)
                        && glyph.get(gx as u32, gy as u32)
                    {
                        self.fb[self.cursor.0 + self.cursor.1 * self.width] = self.foreground;
                    }
                    self.cursor.0 += 1;
                }
            }
            self.cursor.0 = MARGIN;
            self.cursor.1 += 1;
        }
    }

    fn dump_ppm(&mut self, file: &str) -> Result<(), anyhow::Error> {
        let mut f = std::fs::File::create(file)?;
        writeln!(f, "P6")?;
        writeln!(
            f,
            "{} {}",
            self.width * self.scale,
            self.height * self.scale
        )?;
        writeln!(f, "255")?;
        for y in 0..self.height * self.scale {
            for x in 0..self.width * self.scale {
                let c = self.fb[(y / self.scale) * self.width + (x / self.scale)];
                f.write_all(&[(c >> 16) as u8, (c >> 8) as u8, c as u8])?;
            }
        }

        Ok(())
    }

    fn dump_ascii(&mut self) {
        for y in MARGIN..self.height {
            // Let's suppress trailing whitespace
            let mut last = MARGIN;
            for x in MARGIN..self.width {
                if self.fb[y * self.width + x] != BACKGROUND {
                    last = x;
                }
            }

            for x in MARGIN..=last {
                if self.fb[y * self.width + x] == BACKGROUND {
                    print!("  ");
                } else {
                    print!("##");
                }
            }
            println!();
        }
    }

    fn present(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.fb, self.width, self.height)
                .unwrap();
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cfg: Config = argh::from_env();
    let font = bdf::open(cfg.font).unwrap(); // XXX propagate error
    let lines = if cfg.default {
        " !\"#$%&'()*+,-./0123456789:;<=>?\n\
         @ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_\n\
	 `abcdefghijklmnopqrstuvwxyz{|}~\n\
         \n\
	 illegal1 = 0Oo    oO08 iIlL1\n\
	 The quick brown fox jumps over the lazy dog.\n\
	 \n\
	 #define F() (a && b || !c == y^z ? 42 : 12)\n\
	 $a % ($b * $c) ok? YMCA a[42]-A[43] {~c; y}\n\
	 <A> (A) [A] {A}\n\
	 g9qCGQ ~-+=>\n\
	 ({[<>]})\n\
	 {A} */"
            .lines()
            .map(|r| r.to_string())
            .collect()
    } else {
        cfg.text
    };

    let height = lines.len() * 8 + MARGIN * 2;
    let width = lines.iter().map(|x| x.len()).max().unwrap_or(0) * 5 + MARGIN * 2;

    let mut context = Context::new(font, width, height, cfg.scale)?;

    if let Some(fg_str) = cfg.background {
        context.set_background(u32::from_str_radix(&fg_str, 16)?);
    }

    if let Some(fg_str) = cfg.foreground {
        context.set_foreground(u32::from_str_radix(&fg_str, 16)?);
    }

    for line in lines {
        context.renderline(&line);
    }

    context.present();

    if let Some(file) = cfg.ppmdump {
        context.dump_ppm(&file)?;
    }

    if cfg.asciidump {
        context.dump_ascii();
    }

    Ok(())
}
