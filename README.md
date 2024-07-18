# The ALUMET Developer Book

This repository contains a guide for those who want to extend Alumet with plugins.

The book is made with [mdBook](https://rust-lang.github.io/mdBook/).

## Generating the book

First, you need to [install mdBook](https://rust-lang.github.io/mdBook/guide/installation.html#installation).

Then, run `mdbook`.
Example:
```sh
cd user
mdbook serve --open
```

As explained in the [mdBook documentation](https://rust-lang.github.io/mdBook/guide/creating.html#creating-a-book), `serve --open` will build the book, start a local web server and open the book in your default web browser. Modifying the source of the book will automatically reload the web page.
