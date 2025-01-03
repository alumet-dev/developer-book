Welcome to the Alumet developer book! This book is for plugin developers and Alumet contributors.

Here, you can learn:
- how to make Alumet measure new things (to support a new hardware component, for example)
- how to implement a custom model on top of the measured data
- how to implement new outputs for the data (to support a new database, for instance)
- how to participate in the Alumet project

If you want learn how to use Alumet, for example to measure your energy consumption on CPU and GPU, head over to the [user book](https://alumet-dev.github.io/user-book/).

> **🚧 Work in progress**
>
> This book (and the Alumet project as a whole) is a work in progress.
> If you are interested in contributing, please contact us. You can send messages in the ["Discussions" section of the GitHub repository](https://github.com/alumet-dev/alumet/discussions).

## Prerequisites

You need to install Rust, Git, and a code editor/IDE.

⚠️ A recent version of Rust is required (**at least 1.76** for now). You can run `rustc --version` to check your version. The easiest way to install a recent version of Rust is to use [rustup](https://rustup.rs/).

To write Alumet plugins, a basic understanding of the Rust language is required. For simple plugins, you will _not_ need advanced features such as `Send` or `async`. Fundamental notions such as ownership, structures, packages and error handling will be useful.

Are you ready? Let's measure!
