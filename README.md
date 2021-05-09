# Distort images using [Shepard’s algorithm](https://en.wikipedia.org/wiki/Inverse_distance_weighting#Shepard's_method )


Inspired by [ImageMagick core distort command](https://github.com/ImageMagick/ImageMagick/blob/2747ccfc1044fc3da6d32ff1ebbca5e926fcf602/MagickCore/distort.c#L2817).

## Usage
```
shepards <INPUT> <OUTPUT> --points <Point matches>
```
## Example
We want to move the pixel at position (128, 128) to (256, 256) without moving the pixels located at (0,0) and (512,512).
```
shepards input.png output.png --points '128,128 256,256 0,0 0,0 512,512 512,512'
```

| input.png 	| output.png       	|
|------------	|------------------	|
|![output](https://user-images.githubusercontent.com/56520994/117581302-1ceb5800-b0ca-11eb-9439-3bad5c4f5950.png)|![output](https://user-images.githubusercontent.com/56520994/117526570-1d81d280-af94-11eb-83de-dc862731acf1.png)|
