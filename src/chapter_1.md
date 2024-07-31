# Chapter 1

## Create an ALUMet plugin step by step

The best way to get a good understanding of how ALUMet's Plugin works is to do it yourself. So this chapter will create an input plugin, which
read a random byte from a file.

### Create the plugin

In order to create the plugin, we need to initialize a new library. So first, go to the root directory of ALUMet. You should have different
folders containing the different plugins:

```bash
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── LICENSE.fr.txt
├── README.md
├── alumet/
├── alumet-api-dynamic/
├── alumet-api-macros/
├── alumet-config.toml
├── alumet-output.csv
├── app-agent/
├── app-relay-collector/
├── plugin-csv/
├── plugin-influxdb/
├── plugin-nvidia/
├── plugin-oar2/
├── plugin-perf/
├── plugin-rapl/
├── plugin-relay/
├── plugin-socket-control/
├── target/
├── test-dynamic-plugin-c/
├── test-dynamic-plugin-rust/
└── test-dynamic-plugins/
```

So let's create our plugin using:

```bash
cargo init --lib my-plugin
```

Now, go to the Cargo.toml at the root and you should see this new library:

```toml
[...]
members = [
    "alumet",
    [...]
    "my-plugin",
]
```

Now, you can fulfil the TOML of the newly created library with data you want. For example:

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
alumet = { path = "../alumet" }
...
```

### Implement MyPlugin

Let's go to the newly created folder containing the new library. We will use the lib.rs file.

To define our plugin, we need to create a Rust structure: **MyPlugin**. This structure will contain all necessary for the plugin to work.
Let's take an easy structure having 2 fields: config and metrics. Config will contain the configuration of the plugin and metrics which
will contain all related metrics.

```rust 
{{#rustdoc_include plugin_example.rs:MyPlugin_Struct}}
```

Let's define the Metrics structure:

```rust
{{#rustdoc_include plugin_example.rs:Metrics}}
```

For now, the Metrics structure only contains field: *a_metric*. This is a TypedMetricId and its type is an *u64*

### Implement Config

As you can see, MyPlugin contains a Config value. This Config is a structure where you can define value of configuration for the plugin.
Let's define it:

```rust
{{#rustdoc_include plugin_example.rs:Config}}
```

The poll_interval will be the time between two measurements. Feel free to add new element in the configuration if needed.

For ALUMet a Configuration structure needs to implement the `Default` trait, which define the default value if not modified by the user.
Let's do it:

```rust
{{#rustdoc_include plugin_example.rs:impl_default_config}}
```

The default value of poll_interval is a duration of 1 second.

### Implement AlumetPlugin

First, let's create a MyPluginSource struct:

```rust
{{#rustdoc_include plugin_example.rs:mypluginsource}}
```

We have a structure: **MyPlugin** let's implement the `AlumetPlugin` trait, this will transform our structure in an ALUMet plugin
defining some functions:

- name()
- version()
- init()
- default_config()
- start()
- stop()

Let's define these for our plugin:

```rust
{{#rustdoc_include plugin_example.rs:implAlumetPlugin}}
```

Let's focus on the start function.
We want to create a new metric to match with the Metrics structure's field. In this structure, we have one field: *a_metric*.
First, we create a unit associated with the metric:

```rust
{{#rustdoc_include plugin_example.rs:createPrefixedUnit}}
```

Then, we use the create_metric() function of the alumet::plugin::AlumetStart. We specify the kind of value (u64), the name
of the metric, its unit (created above) and the last argument is the description:

```rust
{{#rustdoc_include plugin_example.rs:createMetric}}
```

Now that we have our metric, we need to add a Source to Alumet.

The MyPluginSource structure will be used as a buffer to retrieve values. We need to add this as ALUMet source:

```rust
{{#rustdoc_include plugin_example.rs:source}}
```

Currently, you should have an error about your initial source, it's because the trait bound
`MyPluginSource: alumet::pipeline::Source` is not satisfied. We are now going to implement `Source` to fix this.

### Implement Source

In this part, we will implement the Source trait for our MyPluginSource structure.

```rust
{{#rustdoc_include plugin_example.rs:implSource}}
```

This function is called by Alumet each time a measure is needed, so it's in this function that we need to retrieve the value.
For this example, let's read data from the **/dev/urandom** file.
Here is the code:

```rust
{{#rustdoc_include plugin_example.rs:readRandom}}
```

> N.b. This will only work on UNIX like OS which does have a file at "/dev/urandom"

We are now able to get the value. The next step is to send this value to ALUMet.
In order to push data to alumet, we first need to create a measurement point and then push it to the MeasurementAccumulator.
I also add as an example an attribute the same as value but divided by 2:

```rust
{{#rustdoc_include plugin_example.rs:measurementPointNew}}
```

So final code of `poll` function is:

```rust,ignore
{{#rustdoc_include plugin_example.rs:pollFunction}}
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

You can now build ALUMet.

## Final code

```rust
{{#rustdoc_include plugin_example.rs:all}}
```
