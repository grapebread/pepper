all:
	cargo run --release && convert face2.ppm face2.png

clean:
	rm -rf *.png *.ppm
