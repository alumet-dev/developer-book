extern crate alumet;

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

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(with = "humantime_serde")]
    poll_interval: Duration,
}

pub struct Metrics {
    a_metric: TypedMetricId<u64>,
}

pub struct MyPlugin {
    config: Config,
    metrics: Option<Metrics>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
        }
    }
}

#[derive(Debug)]
struct MyPluginSource {
    random_byte: TypedMetricId<u64>,
}

impl AlumetPlugin for MyPlugin {
    fn name() -> &'static str {
        "MyPlugin"
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(Some(serialize_config(Config::default())?))
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        let config = deserialize_config(config)?;
        Ok(Box::new(MyPlugin { config, metrics: None }))
    }

    fn start(&mut self, alumet: &mut alumet::plugin::AlumetStart) -> anyhow::Result<()> {
        let my_byte_unit: PrefixedUnit = PrefixedUnit {
            base_unit: Unit::Byte,
            prefix: UnitPrefix::Plain,
        };

        let byte_metric = alumet.create_metric::<u64>("random_byte", my_byte_unit, "Byte randomly get")?;
        self.metrics = Some(Metrics { a_metric: byte_metric });

        let initial_source = Box::new(MyPluginSource {
            random_byte: (self.metrics.as_ref().expect("Can't read byte_metric")).a_metric,
        });

        alumet.add_source(initial_source, TriggerSpec::at_interval(self.config.poll_interval));
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Source for MyPluginSource {
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
