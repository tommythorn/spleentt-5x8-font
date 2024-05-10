images/spleentt.png:
	(cd show.rs;cargo run -- --default --screendump /tmp/xx.ppm)
	pnmtopng /tmp/xx.ppm > /tmp/xx.png
	mv /tmp/xx.png $@
