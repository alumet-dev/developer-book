# Chapter 1

## Create an ALUMet plugin step by step

The best way for a good understanding on how ALUMet's Plugin works is doing by yourself. So this chapter will create an input plugin which
read random byte from a file.

### Create the plugin

In order to create the plugin, we need to initialize a new library. So first, go to the root directory of ALUMet. You should have differents
folders containing the differents plugins:

```bash
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── LICENSE.fr.txt
├── README.md
├── alumet
├── alumet-api-dynamic
├── alumet-api-macros
├── alumet-config.toml
├── alumet-output.csv
├── app-agent
├── app-relay-collector
├── plugin-csv
├── plugin-influxdb
├── plugin-nvidia
├── plugin-oar2
├── plugin-perf
├── plugin-rapl
├── plugin-relay
├── plugin-socket-control
├── target
├── test-dynamic-plugin-c
├── test-dynamic-plugin-rust
└── test-dynamic-plugins
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

### Implement ThePlugin

Let's go to the newly created folder containing the new library. We will use the lib.rs file.

To define our plugin, we need to create a Rust structure: **ThePlugin**. This structure will contain all necessary for the plugin to work.
Let's take an easy structure having 2 fields: config and metrics. Config will contain the configuration of the plugin and metrics which
will contain all related metrics.

```rust
pub struct ThePlugin {
    config: Config,
    metrics: Option<Metrics>,
}
```

Let's define the Metrics structure:

```rust
pub struct Metrics {
    a_metric: TypedMetricId<u64>,
}
```

For now, the Metrics structure only contains field: *a_metric*. This is a TypedMetricId and it's type is a *u64*

### Implement Config

As you can see, ThePlugin contains a Config value. This Config is a structure where you can define value of configuration for the plugin.
Let's define it:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(with = "humantime_serde")]
    poll_interval: Duration,
}
```

The poll_interval will be the time waited before two measurements. Feel free to add new element in the configuration if needed.

For ALUMet a Configuration structure needs to implement the **Default** trait, which define the default value if not modified by the user.
Let's do it:

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
        }
    }
}
```

We have defined a duration for poll_interval at 1 second.

### Implement AlumetPlugin

First, let's create a ThePluginSource struct:

```rust
#[derive(Debug)]
struct ThePluginSource {
    random_byte: TypedMetricId<u64>,
}
```

We have a structure: **ThePlugin** let's implement the **AlumetPlugin** trait, this will transform our structure in a ALUMet plugin
defining some functions:

- fn name()
- fn version()
- fn init()
- fn default_config()
- fn start()
- fn stop()

Let's define these for our plugin:

```rust
impl AlumetPlugin for ThePlugin {
    fn name() -> &'static str {
        "ThePlugin"
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(Some(serialize_config(Config::default())?))
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        let config = deserialize_config(config)?;
        Ok(Box::new(ThePlugin { config, metrics: None }))
        
    }

    fn start(&mut self, alumet: &mut alumet::plugin::AlumetStart) -> anyhow::Result<()> {
        ...
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
```

So we define the name of the plugin, it's version, we use the default config by default and on init, we deserialize the config to
take in count if there is a different config than the default one. The start function is here to register metrics, sources and output
and the stop function is called after all the metrics, sources and output previously registered have been stopped and unregistered.
As you can see, currently, we didn't fulfil the start function, let's focus on it now.
We want to create a new metrics to match with the Metrics structure's field. In this structure, we have one field: *a_metric*.
First, we create a unit associated with the metric:

```rust
let my_byte_unit: PrefixedUnit = PrefixedUnit { base_unit: Unit::Byte, prefix: UnitPrefix::Plain };
```

Then, we use the create_metric() function of the alumet::plugin::AlumetStart. We specify the kind of value (u64), the name
of the metric, it's unit (created above), and the last argument is the description:

```rust
let byte_metric = alumet.create_metric::<u64>(
        "random_byte",
        my_byte_unit,
        "Byte randomly get",
    )?;
self.metrics = Some(Metrics {
    a_metric: byte_metric
});
```

Now we have our metric, we need to add a Source to Alumet.

The ThePluginSource structure will be used as buffer to retrieve values. We need to add this as ALUMet source:

```rust
let initial_source = Box::new(ThePluginSource {
    random_byte: (self.metrics.expect("Can't read byte_metric")).a_metric,
});

alumet.add_source(initial_source, TriggerSpec::at_interval(self.config.poll_interval));
```

In the code above, first, we create a source from ThePluginSource structure and then we add it to the alumet sources, adding the poll_interval
value previously defined in the config.

Currently, you should have an error about your initial source, it's because the trait bound
**ThePluginSource: alumet::pipeline::Source** is not satisfied. Go to implement source to satisfy it.

### Implement Source

In this part, we will implement the Source trait for our ThePluginSource structure.

```rust
impl Source for ThePluginSource {
    fn poll(&mut self, measurements: &mut MeasurementAccumulator, timestamp: Timestamp) -> Result<(), PollError> {
        ...
        Ok(())
    }
}
```

This function is called by Alumet each time a measure is needed, so it's in this function that we need to retrieve value.
For this example, let's read data from the **/dev/urandom** file.
Here is the code:

```rust
let mut rng = File::open("/dev/urandom")?; // Open the "/dev/urandom" file to obtain random data

let mut buffer = [0u8; 8]; // Create a mutable buffer of type [u8; 8] (an array of 8 unsigned 8-bit integer)
rng.read_exact(&mut buffer)?; // Read enough byte from the file and store the value in the buffer
let value: u64 = u64::from_le_bytes(buffer);
```

> N.b. This will only work on UNIX like OS which does have a file at "/dev/urandom"

We are now able to get the value. The next step is to send this value to ALUMet.
In order to push data to alumet, we first need to create a measurement point and then push it to the MeasurementAccumulator.
I also add as an example an attribute the same as value but divided by 2 :

```rust
let my_meas_pt = MeasurementPoint::new(
    timestamp,
    self.random_byte,
    Resource::LocalMachine,
    ResourceConsumer::LocalMachine,
    value,
).with_attr("divided", value.div_euclid(2));
measurements.push(my_meas_pt);
```

### Add to the app-agent

To add to the app-agent and so use the plugin, several steps are needed.

#### Add the plugin to the cargo.toml

It's very easy, just add your plugin library with a matching version in the toml file:

```toml
...
[dependencies]
...
my-plugin = {version= "0.1.0", path = "../my-plugin"}
```

#### Import in the app-agent

In the app-agent main file, import using use:

```rust
use my_plugin::ThePlugin;
```

And then add this newly imported plugin to the statics_plugins macro:

```rust
let plugins = static_plugins![ThePlugin, CsvPlugin, SocketControlPlugin];
```

In this example, we have 3 plugins used by the app-agent:

- ThePlugin
- CsvPlugin
- SocketControlPlugin

You can now build ALUMet.

## Final code

```rust
use alumet::{
    measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp},
    metrics::TypedMetricId,
    pipeline::{trigger::TriggerSpec, PollError, Source},
    plugin::{
        rust::{deserialize_config, serialize_config, AlumetPlugin},
        ConfigTable,
    },
    resources::{Resource, ResourceConsumer},
    units::{PrefixedUnit, Unit, UnitPrefix},
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, time::Duration};

pub struct ThePlugin {
    config: Config,
    metrics: Option<Metrics>,
}

pub struct Metrics {
    a_metric: TypedMetricId<u64>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(with = "humantime_serde")]
    poll_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
        }
    }
}

#[derive(Debug)]
struct ThePluginSource {
    random_byte: TypedMetricId<u64>,
}

impl AlumetPlugin for ThePlugin {
    fn name() -> &'static str {
        "ThePlugin"
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(Some(serialize_config(Config::default())?))
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        let config = deserialize_config(config)?;
        Ok(Box::new(ThePlugin { config, metrics: None }))
    }

    fn start(&mut self, alumet: &mut alumet::plugin::AlumetStart) -> anyhow::Result<()> {
        let my_byte_unit: PrefixedUnit = PrefixedUnit {
            base_unit: Unit::Byte,
            prefix: UnitPrefix::Plain,
        };

        let byte_metric = alumet.create_metric::<u64>("random_byte", my_byte_unit, "Byte randomly get")?;
        self.metrics = Some(Metrics { a_metric: byte_metric });

        let initial_source = Box::new(ThePluginSource {
            random_byte: (self.metrics.as_ref().expect("Can't read byte_metric")).a_metric,
        });

        alumet.add_source(initial_source, TriggerSpec::at_interval(self.config.poll_interval));
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Source for ThePluginSource {
    fn poll(&mut self, measurements: &mut MeasurementAccumulator, timestamp: Timestamp) -> Result<(), PollError> {
        let mut rng = File::open("/dev/urandom")?; // Open the "/dev/urandom" file to obtain random data

        let mut buffer = [0u8; 8]; // Create a mutable buffer of type [u8; 8] (an array of 8 unsigned 8-bit integer)
        rng.read_exact(&mut buffer)?; // Read enough byte from the file and store the value in the buffer
        let value = u64::from_le_bytes(buffer);

        // Print the value of the first byte in the buffer
        println!("Random u8: {}", buffer[0]);
        let my_meas_pt = MeasurementPoint::new(
            timestamp,
            self.random_byte,
            Resource::LocalMachine,
            ResourceConsumer::LocalMachine,
            value,
        )
        .with_attr("double", value.div_euclid(2));
        measurements.push(my_meas_pt);

        Ok(())
    }
}

```
