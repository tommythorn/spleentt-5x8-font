# Spleentt

![spleentt](images/spleentt.png  "All ASCII characters in this (spleentt) font, with code snippets")

While considering a project with a 320 × 128 LCD, I needed the
smallest possible readable font that would work here.  The best two
candidates I found were Spleen (5×8) and Creep/Creep2 (5×9).  Each
had their own strengths and issues, so I decided to create a new font,
mostly based on Spleen.

All fonts are compromises and the compromises become more severe as we
reduce the available resolution.  The inherent tension between
improving the visual appearence and respecting the inter-character
spacing is core to all the changes made here.

While stretching glyphs might make them prettier in isolation, once
placed next to other glyphs, the lost whitespace makes for more
difficult reading.  For this reason, all but two horizontal space
violators were eliminated (`#` and `+` remains).

There are only a few vertical violations as we leave the descentor
(the bottom-most row) free, except for glyphs `g`, `j`, `p`, `q`, and
`Q`.  Notably, none of the parenthesis, brackets, braces descend
there.

At this point about half of the ASCII glyphs from the original Spleen font
have been redesigned.

## Original

![spleen](images/spleen.png  "All ASCII characters in the original spleen font")

# Acknowledgments

This wouldn't have been possible without the amazing work of Romeo Van
Snick and Frederic Cambus

* Spleen: https://github.com/fcambus/spleen
* Creep: https://github.com/romeovs/creep
* The X11 fixed fonts
* And _even_ tinier https://github.com/toitware/toit-font-clock
