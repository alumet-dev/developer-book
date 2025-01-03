# Your first plugin

The best way to get a good understanding of how Alumet plugins work is to crate one yourself!
In this chapter, you will create your first plugin, which will produce random bytes.

## Creating the plugin crate

The first thing to do is to initialize a new crate for the plugin. Alumet plugins are not executables by themselves, they are library crates.

You have two options:
1. clone the official Alumet repository and develop there (best if you want to contribute to Alumet)
2. work in your own repository (best if you want to be independent)

### Creating the plugin in a clone of the official repository

Clone the Alumet repository with SSH:
```sh
git clone git@github.com:alumet-dev/developer-book.git
```

Or with HTTPS:
```sh
git clone https://github.com/alumet-dev/developer-book.git
```

Open the root directory of Alumet. You should see several files and folders:

```bash
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── LICENSE.fr.txt
├── README.md
├── alumet/
├── app-agent/
├── plugin-csv/
├── plugin-nvidia/
├── plugin-rapl/
├── plugin-relay/
├── target/
├── ...
```

Let's make a crate for your plugin! By convention, plugins contained in the main repository should be prefixed with `plugin-`:

```bash
cargo init --lib plugin-example
```

This will create a new directory named `plugin-example`, with some files in it.
Cargo should also modify the root `Cargo.toml` to add your plugin to the list of `members`, like this:

```toml
members = [
    "alumet",
    # other crates here
    "plugin-example",
]
```

Finally, use `cargo add` to declare some dependencies. Every plugin needs to depend on at least `alumet` and `anyhow`.

```sh
cargo add alumet anyhow
```

Make sure that the `alumet` dependency is local and does _not_ include a version number:

```toml
[package]
name = "plugin-example"
version = "0.1.0"
edition = "2021"

[dependencies]
alumet = { path = "../alumet" } # LOCAL + NO VERSION
anyhow = "1.0.88"
```

### Creating the plugin in a separate repository

Initialize a crate with cargo:

```bash
cargo init --lib plugin-example
```

Finally, use `cargo add` to declare some dependencies. Every plugin needs to depend on at least `alumet` and `anyhow`.

```sh
cargo add alumet anyhow
```

Since your plugin is not in the main repository of Alumet, the dependency on `alumet` will _not_ be local, but rather downloaded from `crates.io`.

## Coding

Now, the fun part: coding your plugin!

### Implementing MyPlugin

Let's go to the newly created folder containing the new library. We will use the **lib.rs** file.

To define our plugin, we need to create a Rust structure: **MyPlugin**. This structure will contain all necessary for the plugin to work.
Let's take an easy structure having 1 fields: `config`. Config will contain the configuration of the plugin.

```rust,ignore 
{{#rustdoc_include ../code/plugin_example.rs:MyPlugin_Struct}}
```

For now, the Metrics structure only contains field: `a_metric`. This is a `TypedMetricId` and its type is an `u64`

### Implement Config

As you can see, MyPlugin contains a Config value. This Config is a structure where you can define value of configuration for the plugin.
Let's define it:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:Config}}
```

The poll_interval will be the time between two measurements. Feel free to add new element in the configuration if needed.

For Alumet a Configuration structure needs to implement the `Default` trait, which define the default value if not modified by the user.
Let's do it:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:impl_default_config}}
```

The default value of `poll_interval` is a duration of 1 second.

### Implement AlumetPlugin

First, let's create a `MyPluginSource` struct:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:MyPluginSource}}
```

We have a structure: **MyPlugin** let's implement the `AlumetPlugin` trait, this will transform our structure in an Alumet plugin
defining some functions:

- name()
- version()
- init()
- default_config()
- start()
- stop()

Let's define these for our plugin:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:implAlumetPlugin}}
```

Let's focus on the `start` function.
We want to create a new metric to match with the Metrics structure's field. In this structure, we have one field: `a_metric`.
We use the `create_metric()` function of the `alumet::plugin::AlumetStart`. We specify the kind of value (`u64`), the name
of the metric, its unit and the last argument is the description:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:createMetric}}
```

Now that we have our metric, we need to add a Source to Alumet.

The `MyPluginSource` structure will be used as a buffer to retrieve values. We need to add this as Alumet source:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:source}}
```

Currently, you should have an error about your initial source, it's because the trait bound
`MyPluginSource: alumet::pipeline::Source` is not satisfied. We are now going to implement `Source` to fix this.

### Implement Source

In this part, we will implement the Source trait for our `MyPluginSource` structure.

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:implSource}}
```

This function is called by Alumet each time a measure is needed, so it's in this function that we need to retrieve the value.
For this example, let's read data from the **/dev/urandom** file.
Here is the code:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:readRandom}}
```

> N.b. This will only work on UNIX like OS which does have a file at "/dev/urandom"

We are now able to get the value. The next step is to send this value to Alumet.
In order to push data to alumet, we first need to create a measurement point and then push it to the MeasurementAccumulator.
I also add as an example an attribute the same as value but divided by 2:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:measurementPointNew}}
```

So final code of `poll` function is:

```rust,ignore
{{#rustdoc_include ../code/plugin_example.rs:pollFunction}}
```

### Add to the app-agent

To use the plugin, we need to add it to the app-agent. To do so, several steps are needed.

#### Add the plugin to the cargo.toml

It's very easy, just add your plugin library with a matching version in the toml file:

```toml
[dependencies]
my-plugin = {version= "0.1.0", path = "../my-plugin"}
```

or by running:

```bash
cargo add my-plugin --path ../my-plugin
```

#### Import in the app-agent

In the app-agent main file, import using use:

```rust,ignore
use my_plugin::MyPlugin;
```

And then add this newly imported plugin to the statics_plugins macro:

```rust,ignore
let plugins = static_plugins![MyPlugin, CsvPlugin, SocketControlPlugin];
```

In this example, we have 3 plugins used by the app-agent:

- MyPlugin
- CsvPlugin
- SocketControlPlugin

You can now build Alumet.

## Good practices

In this example, all the code is in one file. As a real plugin could be more complex with more code, separate your 
code in several files is a good option. You can separate as you prefer but here is an example.
- File1: All about the plugin,...
- File2: All about the poll function, creation of measurementPoint,...  
- File3: All about the value, how to retrieve, process,.. them.

## Final code

```rust
{{#rustdoc_include ../code/plugin_example.rs:all}}
```
