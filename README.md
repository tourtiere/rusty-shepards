# Distort images using Shepard’s algorithm


Inspired by [ImageMagick core distort command](https://github.com/ImageMagick/ImageMagick/blob/2747ccfc1044fc3da6d32ff1ebbca5e926fcf602/MagickCore/distort.c#L2817).

## Usage
```
shepards <INPUT> <OUTPUT> --points <Point matches>
```
## Example
```
shepards input.png output.png --points '0,0 0,0 128,128 256,256 512,512 512,512'
```

| input.png 	| output.png       	|
|---------	|------------------	|
| 	|  	|

