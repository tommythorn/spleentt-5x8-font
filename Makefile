.PHONY: release

release: SpleenttMedium-8.bdf SpleenttMedium.psf images/spleentt.png

#SpleenttMedium-8.bdf: SpleenttMedium.sfd
#	 (cd padfont;cargo -q r -r -- < ../SpleenttMedium-8.bdf > ../SpleenttMedium-8.bdf.padded && mv ../SpleenttMedium-8.bdf.padded ../SpleenttMedium-8.bdf)

images/spleentt.png: SpleenttMedium-8.bdf
	 (cd show.rs/;cargo -q r -r -- -d --ppmdump /tmp/demo.ppm && pnmtopng < /tmp/demo.ppm > ../images/spleentt.png)

SpleenttMedium.psf: SpleenttMedium-8.bdf
	 (cd genpsf;cargo -q r -r < ../SpleenttMedium-8.bdf > ../SpleenttMedium.psf)
