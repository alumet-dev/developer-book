# The ALUMET Developer Book

This repository contains the "Alumet developer book", a guide for tinkerers, engineers and researchers who want to:
- create new plugins for Alumet (to measure new things, for instance)
- or/and contribute to the project

## Reading the book

The book is available online here: https://alumet-dev.github.io/developer-book

## Generating the book

The book is made with [mdBook](https://rust-lang.github.io/mdBook/).

First, you need to [install mdBook](https://rust-lang.github.io/mdBook/guide/installation.html#installation).

Then, run `mdbook`.
Example:
```sh
mdbook serve --open
```

As explained in the [mdBook documentation](https://rust-lang.github.io/mdBook/guide/creating.html#creating-a-book), `serve --open` will build the book, start a local web server and open the book in your default web browser. Modifying the source of the book will automatically reload the web page.

## Working with diagrams

Some illustrations are made with [draw.io](https://www.drawio.com/).
Every drawio file must be placed in the `diagrams` directory and converted to PNG images with the drawio app.
Use the `regen-diagrams.sh` script to regenerate all the diagrams.
The resulting images are in `src/resources/diagrams`.

In the book (.md files), use the PNG images, not the drawio sources.
