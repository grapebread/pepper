all:
	cargo run --release && convert face.ppm face.png

clean:
	rm -rf *.png *.ppm
