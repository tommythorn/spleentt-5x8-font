# Pad the BDF font

As far as I can tell, fontconfig, used by fbterm, mishandles BDF fonts
and gets confused by the offsets in the BBX.  At least, which my font
works everywhere else, it looks somewhat garbled on fbterm, eg. the
"s" in "ssh" sits flush the with top of the "h" instead of flush with
the bottom.  I speculate that this is a bug in fontconfig and that
padding the font (so all BBX entries have the same offset) will fix it.

This is a quick and dirty attempt at that.
