// extern crate alumet;
// extern crate anyhow;
// extern crate humantime_serde;
// extern crate serde;

// ANCHOR: all
use alumet::plugin::{rust::AlumetPlugin, AlumetPluginStart, ConfigTable};
// ANCHOR: import_elements
use alumet::pipeline::{Output, Source, Transform};
// ANCHOR_END: import_elements

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

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(ExamplePlugin))
    }

    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info("Hello!");
        // ANCHOR: create_source_metric
        let counter_metric = alumet.create_metric::<u64>(
            //                                      ^^^ type
            "example_source_call_counter", // name
            Unit::Unity,                   // unit
            "number of times the example source has been called", // description
        )?;
        // ANCHOR_END: create_source_metric
        // ANCHOR: add_source
        let source = ExampleSource { metric: counter_metric, counter: 0 };
        alumet.add_source(Box::new(source));
        // ANCHOR_END: add_source
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info("Bye!");
        Ok(())
    }
}

// ANCHOR: source
// ANCHOR: source_partial0
struct ExampleSource {
    // ANCHOR_END: source_partial0
    metric: TypedMetricId<u64>,
    counter: u64,
    // ANCHOR: source_partial1
}

impl Source for ExampleSource {
    fn poll(&mut self, acc: &mut MeasurementAccumulator, timestamp: Timestamp) -> Result<(), PollError> {
        // ANCHOR_END: source_partial1
        // ANCHOR: source_poll
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
        // ANCHOR_END: source_poll
        // ANCHOR: source_partial2
    }
}
// ANCHOR_END: source_partial2
// ANCHOR_END: source
// ANCHOR_END: all
