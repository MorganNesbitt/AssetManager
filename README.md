# Asset Manager
==

Project meant to make common tasks around assets easier to do.
Exposes a cli currently to interact.

# Current Commands

1. pack
   - Pack a directory of images to one spritesheet
   - generate a ron file detailing sprite locations
   - Uses sheep packager
2. strip
   - Iterates over images and outputs a separate directory of images without
     extra transparency.
   - Determines the perfect bounding box which still contains all non-zero alpha
     pixel values
