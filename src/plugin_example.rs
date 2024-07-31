extern crate alumet;
extern crate anyhow;
extern crate humantime_serde;
extern crate serde;
// ANCHOR: all
use alumet::{
    measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp},
    metrics::TypedMetricId,
    pipeline::{elements::error::PollError, trigger::TriggerSpec, Source},
    plugin::{
        rust::{deserialize_config, serialize_config, AlumetPlugin},
        AlumetPluginStart, AlumetPostStart, ConfigTable,
    },
    resources::{Resource, ResourceConsumer},
    units::{PrefixedUnit, Unit, UnitPrefix},
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, time::Duration};

// ANCHOR: Config
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(with = "humantime_serde")]
    poll_interval: Duration,
}
// ANCHOR_END: Config
// ANCHOR: Metrics
pub struct Metrics {
    a_metric: TypedMetricId<u64>,
}
// ANCHOR_END: Metrics
// ANCHOR: MyPlugin_Struct
pub struct MyPlugin {
    config: Config,
    metrics: Option<Metrics>,
}
// ANCHOR_END: MyPlugin_Struct
// ANCHOR: impl_default_config
impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
        }
    }
}
// ANCHOR_END: impl_default_config
// ANCHOR: mypluginsource
#[derive(Debug)]
struct MyPluginSource {
    random_byte: TypedMetricId<u64>,
}
// ANCHOR_END: mypluginsource
// ANCHOR: implAlumetPlugin
impl AlumetPlugin for MyPlugin {
    // So we define the name of the plugin.
    fn name() -> &'static str {
        "MyPlugin"
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
        Ok(Box::new(MyPlugin {
            config,
            metrics: None,
        }))
    }

    // The start function is here to register metrics, sources and output.
    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        // ANCHOR: createPrefixedUnit
        let my_byte_unit: PrefixedUnit = PrefixedUnit {
            base_unit: Unit::Byte,
            prefix: UnitPrefix::Plain,
        };
        // ANCHOR_END: createPrefixedUnit
        // ANCHOR: createMetric
        let byte_metric =
            alumet.create_metric::<u64>("random_byte", my_byte_unit, "Byte randomly get")?;
        self.metrics = Some(Metrics {
            a_metric: byte_metric,
        });
        // ANCHOR_END: createMetric
        // ANCHOR: source
        // We create a source from ThePluginSource structure.
        let initial_source = Box::new(MyPluginSource {
            random_byte: (self.metrics.as_ref().expect("Can't read byte_metric")).a_metric,
        });

        // Then we add it to the alumet sources, adding the poll_interval value previously defined in the config.
        alumet.add_source(
            initial_source,
            TriggerSpec::at_interval(self.config.poll_interval),
        );
        // ANCHOR_END: source
        Ok(())
    }
    // The stop function is called after all the metrics, sources and output previously
    // registered have been stopped and unregistered.
    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
// ANCHOR_END: implAlumetPlugin
// ANCHOR: implSource
impl Source for MyPluginSource {
    // ANCHOR: pollFunction
    fn poll(
        &mut self,
        measurements: &mut MeasurementAccumulator,
        timestamp: Timestamp,
    ) -> Result<(), PollError> {
        // ANCHOR: readRandom
        let mut rng = File::open("/dev/urandom")?; // Open the "/dev/urandom" file to obtain random data

        let mut buffer = [0u8; 8]; // Create a mutable buffer of type [u8; 8] (an array of 8 unsigned 8-bit integer)
        rng.read_exact(&mut buffer)?; // Read enough byte from the file and store the value in the buffer
        let value = u64::from_le_bytes(buffer);
        // ANCHOR_END: readRandom
        // ANCHOR: measurementPointNew
        let my_meas_pt = MeasurementPoint::new(
            timestamp,
            self.random_byte,
            Resource::LocalMachine,
            ResourceConsumer::LocalMachine,
            value,
        )
        .with_attr("double", value.div_euclid(2));
        measurements.push(my_meas_pt);
        // ANCHOR_END: measurementPointNew

        Ok(())
    }
    // ANCHOR_END: pollFunction
}
// ANCHOR_END: implSource
// ANCHOR_END: all