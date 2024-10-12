//! Generate PSF font from the BDF font
//!
//! This hack was borne out of necessity as bdf2pcf can only (even
//! barely?) handle fonts that are multiples of 8 pixels wide

use std::io::Write;
use std::mem;
use std::slice;

const PSF_FONT_MAGIC: u32 = 0x864ab572;

#[repr(C)]
struct PsfFontHeader {
    magic: u32,         /* magic bytes to identify PSF */
    version: u32,       /* zero */
    headersize: u32,    /* offset of bitmaps in file, 32 */
    flags: u32,         /* 0 if there's no unicode table */
    numglyph: u32,      /* number of glyphs */
    bytesperglyph: u32, /* size of each glyph */
    height: u32,        /* height in pixels */
    width: u32,         /* width in pixels */
}

fn main() -> anyhow::Result<()> {
    let mut w: isize;
    let mut h: isize;
    let mut xd: isize = 0;
    let mut yd: isize;
    let mut bitmap = false;
    let mut bm = [0u8; 8];
    let mut bm_index = 0usize;
    let mut current_char = 0;

    // Sorry, we only handle ASCII for now, and barely
    let mut font = [[0u8; 8]; 256];

    let lines = std::io::stdin().lines();
    for line in lines {
        let line = line.unwrap();
        let mut words = line.split(' ');
        let first_word = words.next();
        match first_word {
            Some("ENCODING") => {
                let v: Vec<usize> = words.map(|s| s.parse::<usize>().unwrap()).collect();
                assert_eq!(v.len(), 1);
                current_char = v[0];
            }

            Some("BBX") => {
                let v: Vec<isize> = words.map(|s| s.parse::<isize>().unwrap()).collect();
                [w, h, xd, yd] = v.try_into().unwrap(); // XXX should convert to anyhow error somehow

                // "BBX w h xd yd" is a bit confusing.  The bit map
                // that follows has diamensions w x h bits xd is the
                // horizontal offset into the cell yd is the vertical
                // offset(?).  However yd is counting from the bottom
                // and starts at -1!
                // What it all means is that bm_index is 8 - h + yd???

                // Some examples:
                // ']': BBX 2 7 1 0    => index 0    (8-7-0-1)
                // '_': BBX 4 1 0 -1   => index 7    (8-1--1-1)
                // '`': BBX 2 2 1 5    => index 0!!  (8-2-5-1)

                assert!(w <= 5);
                bm_index = if 8 - h - yd - 1 < 0 {
                    0
                } else {
                    (8 - h - yd - 1) as usize
                };
                assert!(
                    bm_index < 8,
                    "bm_index {bm_index} is curiously large (h {h} yd {yd})"
                );
            }

            Some("BITMAP") => {
                bm = [0; 8];
                // -1  -> push 7
                // _ 5 _ 0 => push 2, post append 1
                bitmap = true;
            }

            Some("ENDCHAR") => {
                if (0..256).contains(&current_char) {
                    for (i, b) in bm.iter().enumerate() {
                        font[current_char][i] = *b;
                    }
                }
                bitmap = false;
            }

            Some(w) => {
                if bitmap {
                    assert!(bm_index < 8, "bm_index {bm_index} is curiously large");
                    let v: u8 = u8::from_str_radix(w, 16)?;
                    bm[bm_index] = v >> xd;
                    bm_index += 1;
                }
            }
            _ => {}
        }
    }

    // Dump the font
    let header = PsfFontHeader {
        magic: PSF_FONT_MAGIC,
        version: 0,
        headersize: 32,
        flags: 0,
        numglyph: 256, // This is a lie
        bytesperglyph: 8,
        height: 8,
        width: 5,
    };

    let p: *const PsfFontHeader = &header; // the same operator is used as with references
    let p: *const u8 = p as *const u8; // convert between pointer types
    let s: &[u8] = unsafe { slice::from_raw_parts(p, mem::size_of::<PsfFontHeader>()) };

    std::io::stdout().write_all(s)?;
    for c in 0..256 {
        std::io::stdout().write_all(&font[c])?;
    }

    Ok(())
}
