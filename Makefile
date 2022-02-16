all:
	cargo run && convert image.ppm image.png

clean:
	rm -rf *.png *.ppm
