# Error handling

## Anyhow

Alumet uses [`anyhow`](https://crates.io/crates/anyhow) to simplify error handling. It provides a "universal" error type `anyhow::Error`, which can wrap any error that implements the standard trait `std::error::Error`. In most cases, we simply replace `Result<T, E>` with `anyhow::Result<T>`.

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
fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
    if true { // for testing
        return Err(anyow!("manual error here));
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

As an exercise, modify your source to fail:
```diff
fn poll(...) -> Result<(), PollError> {
+   return Err(anyhow!("cannot poll").into());
    // ...
}
```
