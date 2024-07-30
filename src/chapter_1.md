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

Now, go to the Cargo.toml at the root, and you should see this new library:

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

```rust,ignore
pub struct ThePlugin {
    config: Config,
    metrics: Option<Metrics>,
}
```

Let's define the Metrics structure:

```rust
# extern crate alumet;
# use alumet::metrics::TypedMetricId;

pub struct Metrics {
    a_metric: TypedMetricId<u64>,
}
```

For now, the Metrics structure only contains field: *a_metric*. This is a TypedMetricId and its type is an *u64*

### Implement Config

As you can see, ThePlugin contains a Config value. This Config is a structure where you can define value of configuration for the plugin.
Let's define it:

```rust
# extern crate serde;
# extern crate humantime_serde;
# use serde::{Deserialize, Serialize};
# use std::time::Duration;
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(with = "humantime_serde")]
    poll_interval: Duration,
}
```

The poll_interval will be the time between two measurements. Feel free to add new element in the configuration if needed.

For ALUMet a Configuration structure needs to implement the `Default` trait, which define the default value if not modified by the user.
Let's do it:

```rust,ignore
impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
        }
    }
}
```

The default value of poll_interval is a duration of 1 second.

### Implement AlumetPlugin

First, let's create a ThePluginSource struct:

```rust
# extern crate alumet;
# use alumet::metrics::TypedMetricId;
#[derive(Debug)]
struct ThePluginSource {
    random_byte: TypedMetricId<u64>,
}
```

We have a structure: **ThePlugin** let's implement the `AlumetPlugin` trait, this will transform our structure in an ALUMet plugin
defining some functions:

- name()
- version()
- init()
- default_config()
- start()
- stop()

Let's define these for our plugin:

```rust,ignore
impl AlumetPlugin for ThePlugin {
    // So we define the name of the plugin.
    fn name() -> &'static str {
        "ThePlugin"
    }

    // We also define it's version.
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    // We use the default config by default and on initialization.
    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(Some(serialize_config(Config::default())?))
    }

    // We also use the default config on initialization and we deserialize the config
    // to take in count if there is a different config than the default one.
    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        let config = deserialize_config(config)?;
        Ok(Box::new(ThePlugin { config, metrics: None }))
        
    }

    // The start function is here to register metrics, sources and output.
    fn start(&mut self, alumet: &mut alumet::plugin::AlumetPluginStart) -> anyhow::Result<()> {
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

    // The stop function is called after all the metrics, sources and output previously 
    // registered have been stopped and unregistered.
    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
```

As you can see, currently the start function is empty, let's fill it now.
We want to create a new metric to match with the Metrics structure's field. In this structure, we have one field: *a_metric*.
First, we create a unit associated with the metric:

```rust
# extern crate alumet;
# use alumet::units::{PrefixedUnit, Unit, UnitPrefix};
let my_byte_unit: PrefixedUnit = PrefixedUnit { base_unit: Unit::Byte, prefix: UnitPrefix::Plain };
```

Then, we use the create_metric() function of the alumet::plugin::AlumetStart. We specify the kind of value (u64), the name
of the metric, its unit (created above) and the last argument is the description:

```rust,ignore
let byte_metric = alumet.create_metric::<u64>(
        "random_byte",
        my_byte_unit,
        "Byte randomly get",
    )?;
self.metrics = Some(Metrics {
    a_metric: byte_metric
});
```

Now that we have our metric, we need to add a Source to Alumet.

The ThePluginSource structure will be used as a buffer to retrieve values. We need to add this as ALUMet source:

```rust,ignore
// We create a source from ThePluginSource structure.
let initial_source = Box::new(ThePluginSource {
    random_byte: (self.metrics.expect("Can't read byte_metric")).a_metric,
});
// Then we add it to the alumet sources, adding the poll_interval value previously defined in the config.
alumet.add_source(initial_source, TriggerSpec::at_interval(self.config.poll_interval));
```

Currently, you should have an error about your initial source, it's because the trait bound
`ThePluginSource: alumet::pipeline::Source` is not satisfied. We are now going to implement `Source` to fix this.

### Implement Source

In this part, we will implement the Source trait for our ThePluginSource structure.

```rust
# extern crate serde;
# extern crate humantime_serde;
# extern crate anyhow;
# extern crate alumet;
# use alumet::{
#     measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp},
#     metrics::TypedMetricId,
#     pipeline::{elements::error::PollError, trigger::TriggerSpec, Source},
#     plugin::{
#         rust::{deserialize_config, serialize_config, AlumetPlugin},
#         ConfigTable,
#     },
#     resources::{Resource, ResourceConsumer},
#     units::{PrefixedUnit, Unit, UnitPrefix},
# };
# use serde::{Deserialize, Serialize};
# use std::{fs::File, io::Read, time::Duration};
# 
#[derive(Debug)]
# struct ThePluginSource {
#     random_byte: TypedMetricId<u64>,
# }
impl Source for ThePluginSource {
    fn poll(&mut self, measurements: &mut MeasurementAccumulator, timestamp: Timestamp) -> Result<(), PollError> {
        let mut rng = File::open("/dev/urandom")?; // Open the "/dev/urandom" file to obtain random data

        let mut buffer = [0u8; 8]; // Create a mutable buffer of type [u8; 8] (an array of 8 unsigned 8-bit integer)
        rng.read_exact(&mut buffer)?; // Read enough byte from the file and store the value in the buffer
        let value = u64::from_le_bytes(buffer);

        // Print the value of the first byte in the buffer
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

This function is called by Alumet each time a measure is needed, so it's in this function that we need to retrieve the value.
For this example, let's read data from the **/dev/urandom** file.
Here is the code:

```rust,ignore
let mut rng = File::open("/dev/urandom")?; // Open the "/dev/urandom" file to obtain random data

let mut buffer = [0u8; 8]; // Create a mutable buffer of type [u8; 8] (an array of 8 unsigned 8-bit integer)
rng.read_exact(&mut buffer)?; // Read enough byte from the file and store the value in the buffer
let value: u64 = u64::from_le_bytes(buffer);
```

> N.b. This will only work on UNIX like OS which does have a file at "/dev/urandom"

We are now able to get the value. The next step is to send this value to ALUMet.
In order to push data to alumet, we first need to create a measurement point and then push it to the MeasurementAccumulator.
I also add as an example an attribute the same as value but divided by 2:

```rust,ignore
let my_meas_pt = MeasurementPoint::new(
    timestamp,
    self.random_byte,
    Resource::LocalMachine,
    ResourceConsumer::LocalMachine,
    value,
).with_attr("divided", value.div_euclid(2));
measurements.push(my_meas_pt);
```

So final code of `poll` function is:

```rust,ignore
fn poll(&mut self, measurements: &mut MeasurementAccumulator, timestamp: Timestamp) -> Result<(), PollError> {
        let mut rng = File::open("/dev/urandom")?; // Open the "/dev/urandom" file to obtain random data

        let mut buffer = [0u8; 8]; // Create a mutable buffer of type [u8; 8] (an array of 8 unsigned 8-bit integer)
        rng.read_exact(&mut buffer)?; // Read enough byte from the file and store the value in the buffer
        let value = u64::from_le_bytes(buffer);

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
```

### Add to the app-agent

To use the plugin, we need to add it to the app-agent. To do so, several steps are needed.

#### Add the plugin to the cargo.toml

It's very easy, just add your plugin library with a matching version in the toml file:

```toml
[dependencies]
my-plugin = {version= "0.1.0", path = "../my-plugin"}
```

#### Import in the app-agent

In the app-agent main file, import using use:

```rust,ignore
use my_plugin::ThePlugin;
```

And then add this newly imported plugin to the statics_plugins macro:

```rust,ignore
let plugins = static_plugins![ThePlugin, CsvPlugin, SocketControlPlugin];
```

In this example, we have 3 plugins used by the app-agent:

- ThePlugin
- CsvPlugin
- SocketControlPlugin

You can now build ALUMet.

## Final code

```rust
# extern crate serde;
# extern crate humantime_serde;
# extern crate anyhow;
# extern crate alumet;
use alumet::{
    measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp},
    metrics::TypedMetricId,
    pipeline::{elements::error::PollError, trigger::TriggerSpec, Source},
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

    fn start(&mut self, alumet: &mut alumet::plugin::AlumetPluginStart) -> anyhow::Result<()> {
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
