Leptess
=======

[![CircleCI](https://circleci.com/gh/houqp/leptess.svg?style=svg)](https://circleci.com/gh/houqp/leptess)
[![Crates.io](https://img.shields.io/crates/v/leptess.svg)](https://crates.io/crates/leptess)

High level Rust binding for Tesseract and Leptonica.

Low level C API bindings are auto generated using bindgen.


Build dependencies
------------------

Make sure you have Leptonica and Tesseract installed.

For Ubuntu user:

```bash
sudo apt-get install libleptonica-dev libtesseract-dev
```

You will also need to install tesseract language data based on your OCR needs:

```bash
sudo apt-get install tesseract-ocr-eng
```


Usage
-----

Minimal example:

```rust
let api = tesseract::TessApi::new(None, "eng");
let pix = leptonica::pix_read(Path::new("path/page.bmp")).unwrap();
api.set_image(&pix);

println!("{}", api.get_utf8_text().unwrap());

api.destroy();
pix.destroy();
```

For more examples, see `examples` directory.


Development
-----------

Regenerate capi binding:

```
make gen
```

To run tests, you will need at Tesseract 4.x to match what we have in `tests/tessdata/eng.traineddata`. See CircleCI config to see how to replicate the setup.
