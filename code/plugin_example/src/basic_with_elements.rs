use std::fs::File;
use std::io::BufWriter;
use std::time::{Duration, SystemTime};

use alumet::measurement::{
    MeasurementAccumulator, MeasurementBuffer, MeasurementPoint, Timestamp, WrappedMeasurementValue,
};
use alumet::metrics::{MetricId, RawMetricId, TypedMetricId};
use alumet::pipeline::elements::error::{PollError, TransformError, WriteError};
use alumet::pipeline::elements::output::OutputContext;
use alumet::pipeline::elements::transform::TransformContext;
use alumet::pipeline::{trigger, Output, Source, Transform};
use alumet::plugin::{rust::AlumetPlugin, AlumetPluginStart, ConfigTable};
use alumet::resources::{Resource, ResourceConsumer};
use alumet::units::Unit;
use anyhow::Context;

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
        // ANCHOR: create_transform_metric
        // Create a metric for the transform.
        let diff_metric = alumet.create_metric::<u64>(
            "example_source_call_diff",
            Unit::Unity,
            "number of times the example source has been called since the previous measurement",
        )?;
        // ANCHOR_END: create_transform_metric

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

        // ANCHOR: add_transform
        // Create the transform
        let transform = ExampleTransform {
            counter_metric: counter_metric.untyped_id(),
            previous_counter: None,
            diff_metric,
        };

        // Add the transform to the measurement pipeline
        alumet.add_transform(Box::new(transform));
        // ANCHOR_END: add_transform

        // ANCHOR: add_output
        // Open the file and writer
        let writer = BufWriter::new(File::create("alumet-tutorial-output.txt")?);

        // Create the output
        let output = ExampleOutput { writer };

        // Add the output to the measurement pipeline
        alumet.add_blocking_output(Box::new(output));
        // ANCHOR_END: add_output

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

// ANCHOR: config

struct ExampleConfig {
    
}
// ANCHOR_END: config

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

// ANCHOR: transform
// ANCHOR: transform_struct
struct ExampleTransform {
    counter_metric: RawMetricId,
    previous_counter: Option<u64>,
    diff_metric: TypedMetricId<u64>,
}
// ANCHOR_END: transform_struct

// ANCHOR: transform_impl
impl Transform for ExampleTransform {
    fn apply(&mut self, measurements: &mut MeasurementBuffer, _ctx: &TransformContext) -> Result<(), TransformError> {
        // Find the relevant measurement points and update the last known counter.
        let mut latest_counter = None;
        for m in measurements.iter() {
            if m.metric == self.counter_metric {
                let value = match m.value {
                    WrappedMeasurementValue::F64(_) => {
                        unreachable!("wrong counter type, expected u64")
                    }
                    WrappedMeasurementValue::U64(c) => c,
                };
                latest_counter = Some((value, m.timestamp));
            }
        }

        // Compute the difference, if we have enough value to do so (previous and latest)
        if let (Some(previous), Some((latest, t))) = (self.previous_counter, latest_counter) {
            let diff = latest - previous;
            // Push the new measurement to the buffer
            measurements.push(MeasurementPoint::new(
                t,                              // For convenience, we use the timestamp of the latest counter update
                self.diff_metric,               // Use the new metric
                Resource::LocalMachine,         // No specific resource
                ResourceConsumer::LocalMachine, // No specific consumer
                diff,                           // The computed value
            ));
        }

        // Update the internal state, if possible.
        // In the case where there are other sources,
        // the buffer may contain no measurements from our example source.
        if let Some((latest, _)) = latest_counter {
            self.previous_counter = Some(latest);
        }

        Ok(())
    }
}
// ANCHOR_END: transform_impl
// ANCHOR_END: transform

// ANCHOR: output
// ANCHOR: output_struct
struct ExampleOutput {
    writer: BufWriter<File>,
}
// ANCHOR_END: output_struct

impl Output for ExampleOutput {
    fn write(&mut self, measurements: &MeasurementBuffer, ctx: &OutputContext) -> Result<(), WriteError> {
        use std::io::Write; // necessary for the writeln! macro

        for m in measurements.iter() {
            // Get a human-readable time from the timestamp.
            // We later use its Debug implementation to convert it to a string easily.
            let time = SystemTime::from(m.timestamp);

            // The measurement point contains the metric id, but it means nothing to a user.
            // Use the OutputContext to obtain the find the metric definition and obtain its name.
            let metric_name = &ctx
                .metrics
                .by_id(&m.metric)
                .with_context(|| format!("unregistered metric id: {}", m.metric.as_u64()))?
                .name;

            // Convert the value to a string. Multiple types are supported, handle them all.
            let value_str = match m.value {
                WrappedMeasurementValue::F64(x) => x.to_string(),
                WrappedMeasurementValue::U64(x) => x.to_string(),
            };

            // The `resource` and `consumer` are each made of two parts: kind and id.
            let resource_kind = m.resource.kind();
            let resource_id = m.resource.id_display();
            let consumer_kind = m.consumer.kind();
            let consumer_id = m.consumer.id_display();

            // There can be an arbitrary number of key-value attributes, use `Vec::join` to convert it to a single string.
            let attributes_str = m
                .attributes()
                .map(|(key, value)| format!("{key}='{value}'"))
                .collect::<Vec<_>>()
                .join(",");

            // Write one line to the file.
            writeln!(&mut self.writer, "{time:?}: {metric_name} = {value_str}; resource = {resource_kind}/{resource_id}; consumer = {consumer_kind}/{consumer_id}; attributes = [{attributes_str}]")?;
        }
        Ok(())
    }
}
// ANCHOR_END: output
