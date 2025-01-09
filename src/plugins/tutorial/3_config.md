# A First Configuration

The counter source that you have implemented in the previous section uses a fixed polling interval.
Instead of using a hard-coded value, it would be better to provide a configuration option so that we can choose the acquisition frequency of the source before starting the Alumet agent.

On startup, the standard agent reads a configuration file in the [TOML](https://toml.io) format.
It contains some options for the agent, and one section per plugin.
Each plugin is free to declare what it needs in its configuration section, and to process it how it wants to.
The best practice is to deserialize the configuration section to a Rust structure.
This is what you will implement in this chapter.

## Config structure

To serialize and deserialize the configuration section, Alumet uses `serde`, which is the de-facto standard framework for (de)serialization in Rust.
Add it to the dependencies by running this command in the plugin's directory:
```sh
cargo add serde --features derive
```

Then, simply define a new structure for the config and use a `derive` macro to make `serde` generate the deserialization code for you. We also generate the serialization code (by deriving `Serialize`), which you will need soon.
```rust,ignore
#[derive(Serialize, Deserialize)]
struct Config {
    /// Time between each activation of the counter source.
    poll_interval: Duration
}
```

To (de)serialize the duration in a human-readable way, such as `"10s"` for 10 seconds, we need another dependency.
Add it with `cargo`, and modify the `Config` structure to use it.

```sh
cargo add humantime-serde
```

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:config_struct}}
```

## Config loading

As explained in the introduction, each plugin gets its own configuration section.
It is accessible in `init`.

Modify `init` to get your config and store it in the plugin structure.
Doing so will allow you to use the config in `start`.

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:plugin_init}}
```

Of course, you also need to update the plugin structure accordingly.
```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:plugin_struct}}
```

## Default config

Though it is not mandatory, you should provide default values for the configuration of your plugin.

Implement the standard [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) trait for the `Config` struct.

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:config_default_impl}}
```

You can now call `Config::default()` to obtain a `Config` structure filled with default values.
Use this in `default_config` to return your default configuration.

Note that you must use `serialize_config` (`alumet::plugin::rust::serialize_config`) to convert your configuration structure into a `ConfigTable`, which is a "universal" configuration type provided by Alumet. Of course, `serialize_config` internally uses `serde`, that is why it was needed to derive the `Serialize` trait.

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:plugin_default_config}}
```

## Using the config in start

Now that the plugin stores its deserialized config in its structure, you can use it in `start` to change the polling interval of the "counter" source that you have previously implemented.

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:add_source}}
```
