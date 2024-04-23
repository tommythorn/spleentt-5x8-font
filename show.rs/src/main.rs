use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::env;

const WIDTH: usize = 320+8;
const HEIGHT: usize = 200+8;

const BACKGROUND: u32 = 19 << 16 | 119 << 8 | 61;
const FOREGROUND: u32 = 255 << 16 | 240 << 8 | 165;

struct Context {
    window: Window,
    font: bdf::Font,
    cursor: (usize, usize),
    fb: [u32; WIDTH * HEIGHT],
}

impl Context {
    fn new(font: bdf::Font) -> Result<Context, String> {
        let mut window = Window::new(
            "spleentt - hit Escape to exit",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: true,
                scale: Scale::X2,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
        .expect("Unable to Open Window");

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Ok(Context {
            window,
            font,
            cursor: (0, 0),
            fb: [BACKGROUND; WIDTH * HEIGHT],
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
                        self.fb[self.cursor.0 + self.cursor.1 * WIDTH + 4 + 4 * WIDTH] = FOREGROUND;
                    }
                    self.cursor.0 += 1;
                }
            }
            self.cursor.0 = 0;
            self.cursor.1 += 1;
        }
    }

    fn present(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.fb, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}

fn main() -> Result<(), String> {
    let mut context = Context::new(bdf::open("../SpleenttMedium-8.bdf").unwrap())?;

    for arg in env::args().skip(1) {
        context.renderline(&arg);
    }

    for arg in "Hello World!\n\
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
    {
        context.renderline(&arg);
    }

    context.present();

    Ok(())
}
