# img2webp

A simple script to traverse through a directory and make a `.webp` version of source image files.

## Prequisites

* Install [cwebp](https://developers.google.com/speed/webp/docs/cwebp) 

## Limitations

This is just something I built to play around with Rust and to streamline a recurring task for an Android app.  With that, it's currently limited to my specific use case.  Buyer beware.  The script:

 * Only targets `.png` files
 * Has only been tested on MacOS (I use the word _tested_ lightly)
 * Currently doesn't accept any `cwebp` arguments
 * Does not support symbolically linked files

In addition to those tasks I might one day:

* Spawn processes concurrently (though it's pretty quick with ~100 images ranging in size from ~ `500KB` - `2.5MB`)
* Support deleting source files
* Support regex to find files rather than targeting all `png`