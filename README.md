[#](#) esc-pos-lib
A library for the use of an ESC/POS-thermal printer



Test image for the image library is from [here](https://unsplash.com/photos/VQolpulWf2Y).
Free to use with the unsplash license.
Thanks [Taraqur Rahman](https://unsplash.com/@tqrahman) for the picture. 

## Command implementation status (draft)

- Implemented:
  - Select the print density (`Printer::select_print_density`, draft `GS ( K fn=49`)
  - Select the print speed (`Printer::select_print_speed`, draft `GS ( K fn=50`)
  - Print and feed n lines (`Printer::feed` / `Printer::print_and_feed_lines`)
  - Print and reverse feed n lines (`Printer::print_and_reverse_feed_lines`)
  - Print and feed paper (`Printer::print_and_feed_paper`)
  - Print raster bit image (`image::Image::export`, `GS v 0`)
- Not implemented:
  - Buzzer command group
  - QR command: transmit symbol data size (`fn=182`)
  - Real-time / maintenance / peripheral command group from issue
  - Obsolete NV/downloaded bit-image command group (except raster bit image above)
