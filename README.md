# Spleentt

![spleentt](images/spleentt.png  "All ASCII characters in this (spleentt) font, with code snippets")

While considering a project with a 320 x 128 LCD, I needed the
smallest possible readable font that would work here.  The best two
candidates I found where Spleen (5x8) and Creep/Creep2 (5x9).  Each
had their own strenghts and issues, so I decided to create a new font,
mostly based on Spleen.

All fonts are compromises and the compromises become more severe as we
reduce the available resolution.  The inherint tension between improve
the visual appearence and respecting the inter-character spacing is
core to all the changes made here.

While stretching glyphs might make them prettier in isolation, once
placed next to other glyphs, the lost whitespace makes for more
difficult reading.  For this reason, almost all horizontal space
violations were eliminated (`+` still remains).

There are many vertical violations, but as much as possible we leave
the descentor (the bottom-most row) free, only using it for `g`, `j`,
`p`, and `q`.  Notably, none of the parenthesis, brackets, braces
descend there.


At this point most of the ASCII glyphs from the original Spleen font
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
