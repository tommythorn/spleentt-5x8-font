fn main() {
    let mut w: isize;
    let mut h: isize;
    let mut xd: isize = 0;
    let mut yd: isize;
    let mut bitmap = false;
    let mut bm = [0u8; 8];
    let mut bm_index = 0;

    let lines = std::io::stdin().lines();
    for (lineno, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut words = line.split(' ');
        let first_word = words.next();
        match first_word {
            Some("BBX") => {
                let v: Vec<isize> = words.map(|s| s.parse::<isize>().unwrap()).collect();
                [w, h, xd, yd] = v.clone().try_into().unwrap();

                // "BBX w h xd yd" is a bit confusing.  The bit map
                // that follows has dimensions w x h bits, where xd is
                // the horizontal offset into the cell and yd is the
                // vertical offset(?).  However yd is counting from
                // the bottom and starts at -1!  What it all means is
                // that bm_index is 8 - h + yd???

                // Some examples:
                // ']': BBX 2 7 1 0    => index 0    (8-7-0-1)
                // '_': BBX 4 1 0 -1   => index 7    (8-1--1-1)
                // '`': BBX 2 2 1 5    => index 0!!  (8-2-5-1)

                assert!(w <= 5);
                bm_index = 8 - h - yd - 1;

                assert!(-1 <= bm_index && bm_index < 8, "BBX {v:?} -> offset {bm_index}");
                println!("BBX 5 8 0 0");
            }

            Some("BITMAP") => {
                bm = [0; 8];
                // -1  -> push 7
                // _ 5 _ 0 => push 2, post append 1
                bitmap = true;
                println!("{}", line.clone());
            }

            Some("ENDCHAR") => {
                for b in &bm {
                    println!("{b:02X}");
                }

                bitmap = false;
                println!("{}", line.clone());
            }

            Some(w) => {
                if bitmap {
                    if 0 <= bm_index {
                        let v: u8 = u8::from_str_radix(w, 16).unwrap();
                        assert!(bm_index < bm.len() as isize, "{}: in {w}, out of range {bm_index}", lineno + 1);
                        bm[bm_index as usize] = v >> xd;
                        bm_index += 1;
                    }
                } else {
                    println!("{}", line.clone());
                }
            }
            _ => {}
        }
    }
}
