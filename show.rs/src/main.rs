use argh::FromArgs;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::io::prelude::*;

const MARGIN: usize = 4;

const BACKGROUND: u32 = 19 << 16 | 119 << 8 | 61;
const FOREGROUND: u32 = 255 << 16 | 240 << 8 | 165;

#[derive(FromArgs)]
/// Demo the font
struct Config {
    /// give the path to the font
    #[argh(option, default = "String::from(\"../SpleenttMedium-8.bdf\")")]
    font: String,

    /// if given, dump a screenshot here in PPM format
    #[argh(option)]
    screendump: Option<String>,

    /// show the default sample
    #[argh(switch, short = 'd')]
    default: bool,

    /// arguments
    #[argh(positional, greedy)]
    text: Vec<String>,
}

struct Context {
    window: Window,
    font: bdf::Font,
    width: usize,
    height: usize,
    cursor: (usize, usize),
    fb: Vec<u32>,
}

impl Context {
    fn new(font: bdf::Font, width: usize, height: usize) -> anyhow::Result<Context> {
        let mut window = Window::new(
            "spleentt - hit Escape to exit",
            width,
            height,
            WindowOptions {
                resize: true,
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
            fb: vec![BACKGROUND; width * height],
        })
    }

    fn renderline(&mut self, s: &str) {
        let default = self.font.glyphs().get(&'?').unwrap();

        for y in 0..8 {
            for c in s.chars() {
                let glyph = self.font.glyphs().get(&c).unwrap_or(default);
                let bb = glyph.bounds();
                let (height, width) = (bb.height as i32, bb.width as i32); // Sigh

                for x in 0..5 {
                    let gy = y + bb.y - (7 - height);
                    let gx = x - bb.x;

                    if (0..height).contains(&gy)
                        && (0..width).contains(&gx)
                        && glyph.get(gx as u32, gy as u32)
                    {
                        self.fb[self.cursor.0 + self.cursor.1 * self.width] = FOREGROUND;
                    }
                    self.cursor.0 += 1;
                }
            }
            self.cursor.0 = MARGIN;
            self.cursor.1 += 1;
        }
    }

    fn dump(&mut self, file: &str) -> Result<(), anyhow::Error> {
        let mut f = std::fs::File::create(file)?;
        writeln!(f, "P6")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.fb[y * self.width + x];
                // XXX I don't understand the color mess up here
                if c == BACKGROUND {
                    f.write_all(&[0x37, 0x75, 0x43])?;
                } else {
                    f.write_all(&[0xfc, 0xf0, 0xae])?;
                }
            }
        }

        Ok(())
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
        "Hello World!\n\
		illegal1 = 0Oo\n\
		ABCDEFGHIJKLMNOPQRSTUVWXYZ\n\
		abcdefghijklmnopqrstuvwxyz\n\
		0123456789 () {} [] <> @ $\n\
		~ # % ^ & * - = + / ? : ; _ |\n\
		the quick brown fox jumps over the lazy dog.\n\
		\n\
		 !\"#$%&'()*+,-./\n\
		0123456789:;<=>?\n\
		@ABCDEFGHIJKLMNO\n\
		PQRSTUVWXYZ[\\]^_\n\
		`abcdefghijklmno\n\
		pqrstuvwxyz{|}~\n\
		\n\
		#define F() (a && b || !c == y^z ? 42 : 12)\n\
		$a % ($b * $c) ok? YMCA a[42]-A[43] {~c; y}\n\
		<A>\n\
		(A)\n\
		[A]\n\
		oO08 iIlL1\n\
		g9qCGQ ~-+=>\n\
		({[<>]})\n\
		{A} */ THE END."
            .lines()
            .map(|r| r.to_string())
            .collect()
    } else {
        cfg.text
    };

    let height = lines.len() * 8 + MARGIN * 2;
    let width = lines.iter().map(|x| x.len()).max().unwrap_or(0) * 5 + MARGIN * 2;

    let mut context = Context::new(font, width, height)?;

    for line in lines {
        context.renderline(&line);
    }

    context.present();

    if let Some(file) = cfg.screendump {
        context.dump(&file)?;
    }

    Ok(())
}
