use std::time::Duration;

use alumet::measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp};
use alumet::metrics::TypedMetricId;
use alumet::pipeline::elements::error::PollError;
use alumet::pipeline::{trigger, Source};
use alumet::plugin::{rust::AlumetPlugin, AlumetPluginStart, ConfigTable};
use alumet::resources::{Resource, ResourceConsumer};
use alumet::units::Unit;

pub struct ExamplePlugin;

impl AlumetPlugin for ExamplePlugin {
    fn name() -> &'static str {
        "example" // the name of your plugin, in lowercase, without the "plugin-" prefix
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION") // gets the version from the Cargo.toml of the plugin crate
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(None) // no config for the moment
    }

    fn init(_config: ConfigTable) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(ExamplePlugin))
    }

    // ANCHOR: plugin_start
    // ANCHOR: plugin_start_head
    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");
        // ANCHOR_END: plugin_start_head

        // ANCHOR: create_source_metric
        // Create a metric for the source.
        let counter_metric = alumet.create_metric::<u64>(
            //                                      ^^^ type
            "example_source_call_counter",                        // name
            Unit::Unity,                                          // unit
            "number of times the example source has been called", // description
        )?;
        // ANCHOR_END: create_source_metric

        // ANCHOR: add_source
        // Create the source
        let source = ExampleSource {
            metric: counter_metric,
            counter: 0,
        };

        // Configure how the source is triggered: Alumet will call the source every 1s
        let trigger = trigger::builder::time_interval(Duration::from_secs(1)).build()?;

        // Add the source to the measurement pipeline
        alumet.add_source(Box::new(source), trigger);
        // ANCHOR_END: add_source

        // ANCHOR: plugin_start_tail
        Ok(())
    }
    // ANCHOR_END: plugin_start_tail
    // ANCHOR_END: plugin_start

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}

// ANCHOR: source
// ANCHOR: source_struct
struct ExampleSource {
    metric: TypedMetricId<u64>,
    counter: u64,
}
// ANCHOR_END: source_struct

// ANCHOR: source_impl
impl Source for ExampleSource {
    fn poll(&mut self, acc: &mut MeasurementAccumulator, timestamp: Timestamp) -> Result<(), PollError> {
        let n_calls = self.counter;
        self.counter += 1;

        let point = MeasurementPoint::new(
            timestamp,
            self.metric,
            Resource::LocalMachine,
            ResourceConsumer::LocalMachine,
            n_calls, // measured value
        );
        acc.push(point);
        Ok(())
    }
}
// ANCHOR_END: source_impl
// ANCHOR_END: source
