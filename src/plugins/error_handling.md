# Error Handling for Plugins

In the [plugin tutorial](../plugins/tutorial/0_intro.md), we did not need to manage errors in a complicated way because there was almost no source of failure.
Most of our functions returned `Ok(())`, and we used `?` to propagate errors, for example in `start`.

In this chapter, you will discover how to handle errors in more realistic cases.
If you are not familiar with Rust approach to error handling, please [read the corresponding chapter of the Rust book](https://doc.rust-lang.org/book/ch09-00-error-handling.html).

## Anyhow

Alumet uses [`anyhow`](https://crates.io/crates/anyhow) to simplify error handling in the plugin API. It provides a "universal" error type `anyhow::Error`, which can wrap any error that implements the standard trait `std::error::Error`. In most cases, we simply replace `Result<T, E>` with `anyhow::Result<T>`.

Good practices:
- Propagate errors with `?`.
- Add some context, when appropriate, with `with_context` (takes a closure called on error - use this when formatting a string) or `context` (takes a context directly). This is especially useful for errors related to file IO, because they do not include the file path.

Here is an example:

```rs
fn read_file(path: &str) -> Result<String, std::io::error::Error> {
    std::fs::read_to_string(path)
}

fn parse_file_content(content: &str) -> Result<u64, ParseIntError> {
    content.parse()
}

fn f() -> anyhow::Result<u64> {
    let file = "example.txt";
    // Note how `read_file` and `parse_file_content` return different error types,
    // but anyhow treat them exactly the same
    let content = read_file(file).with_context(|| format!("failed to read file {file}"))?;
    let value = parse_file_content(content).with_context(|| format!("invalid content: {content}"))?;
    Ok(value)
}

Try to modify your plugin's `init`:
```rust,ignore
fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
    // Here we use .context because we know the error message at compile-time,
    // there is no formatting.
    std::fs::read_to_string("example.txt").context("failed to read example.txt")?;
    Ok(Box::new(ExamplePlugin))
}
```

It is also possible to create an `anyhow::Error` "manually" with the `anyhow!` macro:
 
```rust,ignore
use anyhow::anyhow;

fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
    if true { // for testing
        return Err(anyhow!("manual error here));
    }
    Ok(Box::new(ExamplePlugin))
}
```

## Pipeline errors

In the pipeline elements (sources, transforms, outputs), Alumet makes a distinction between multiple kinds of errors. In particular, it is useful to distinguish between:
- **fatal errors**, which indicate that the element is broken and cannot be used anymore. If a source, transform or output returns a fatal error, Alumet will discard it.
- **non-fatal errors**, which indicate that the error does not compromise the element and that we can keep it. If a source, transform or output returns a non-fatal error, Alumet will keep it in the pipeline.

The precise semantics depend on the element. See:
- `alumet::pipeline::error::PollError` for sources
- `alumet::pipeline::error::TransformError` for transforms
- `alumet::pipeline::error::WriteError` for outputs

These error types can wrap any `anyhow::Error`, and default to the *fatal* kind.

As an exercise, modify your source to fail with two different approaches:
```diff
fn poll(...) -> Result<(), PollError> {
+   return Err(anyhow!("cannot poll").into());
    // ...
}
```

```diff
use alumet::pipeline::elements::error::PollRetry;

fn poll(...) -> Result<(), PollError> {
+   return Err(anyhow!("cannot poll").retry_poll());
    // ...
}
```

## Panics

As explained in the [Rust book](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html), panics should not be used for reporting "regular" errors such as parsing invalid data.
Panics should be used when you're in a state that cannot be handled, when continuing could be insecure or harmful.

A general rule is: **avoid panicking in your plugin**. Use `Result` instead (see paragraph about Anyhow).
If you panic in plugin's methods like `start` or `stop`, the Alumet agent will crash.
