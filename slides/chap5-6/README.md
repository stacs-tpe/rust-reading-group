# Introduction to Rust

Please see [here](https://github.com/swallez/introduction-to-rust?) for template / further info. 

### How this presentation was prepared

The slides are rendered using [mdBook](http://rust-lang-nursery.github.io/mdBook/), the same tool used to render the [Rust Book](https://doc.rust-lang.org/book/). Along with being a dogfooding experiment, this allows the code examples to be run or even modified from the presentation itself. Live coding within the slideshow is pretty cool!

The default theme was tweaked a bit for slide rendering. The slides are written in a [single markdown file](src/all-slides.md) which is split into individual pages expected by mdbook [using good old `awk`](src/split-slides.sh).

### Updating the content and building the slides

- Make sure you have mdbook installed with `cargo install mdbook`
- Edit `src/all-slides.md`
- Build the slides with:
  
  ```
  (cd src; ./split-slides.sh)
  mdbook build
  ```

### License

The content of this repository is licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).
