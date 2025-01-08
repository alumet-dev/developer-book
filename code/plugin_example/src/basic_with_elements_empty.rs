#![allow(dead_code, unused_variables)]
use alumet::measurement::{MeasurementAccumulator, MeasurementBuffer, Timestamp};
use alumet::pipeline::elements::error::{PollError, TransformError, WriteError};
use alumet::pipeline::elements::output::OutputContext;
use alumet::pipeline::elements::transform::TransformContext;
// ANCHOR: all
use alumet::plugin::{rust::AlumetPlugin, AlumetPluginStart, ConfigTable};
// ANCHOR: import_elements
use alumet::pipeline::{Output, Source, Transform};
// ANCHOR_END: import_elements

// ANCHOR: plugin
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

    // ANCHOR: plugin_start
    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");
        Ok(())
    }
    // ANCHOR_END: plugin_start

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}
// ANCHOR_END: plugin

// ANCHOR: source
struct ExampleSource {
    // TODO
}

impl Source for ExampleSource {
    fn poll(&mut self, acc: &mut MeasurementAccumulator, t: Timestamp) -> Result<(), PollError> {
        todo!()
    }
}
// ANCHOR_END: source

// ANCHOR: transform
struct ExampleTransform {
    // TODO
}

impl Transform for ExampleTransform {
    fn apply(&mut self, measurements: &mut MeasurementBuffer, _ctx: &TransformContext) -> Result<(), TransformError> {
        todo!()
    }
}
// ANCHOR_END: transform

// ANCHOR: output
struct ExampleOutput {
    // TODO
}

impl Output for ExampleOutput {
    fn write(&mut self, measurements: &MeasurementBuffer, ctx: &OutputContext) -> Result<(), WriteError> {
        todo!()
    }
}
// ANCHOR_END: output

// ANCHOR_END: all
