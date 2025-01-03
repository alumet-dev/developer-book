# The ALUMET Developer Book

This repository contains the "Alumet developer book", a guide for tinkerers, engineers and researchers who want to:
- create new plugins for Alumet (to measure new things, for instance)
- or/and contribute to the project

## Reading the book

The book is available online (via GitHub pages) here: https://alumet-dev.github.io/user-book/

## Generating the book

The book is made with [mdBook](https://rust-lang.github.io/mdBook/).

First, you need to [install mdBook](https://rust-lang.github.io/mdBook/guide/installation.html#installation).

Then, run `mdbook`.
Example:
```sh
mdbook serve --open
```

As explained in the [mdBook documentation](https://rust-lang.github.io/mdBook/guide/creating.html#creating-a-book), `serve --open` will build the book, start a local web server and open the book in your default web browser. Modifying the source of the book will automatically reload the web page.
