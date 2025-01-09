# Measuring with sources

For the moment, your plugin doesn't measure anything.
You will now write your first "source" in order to obtain measurement points, that will be passed to the rest of the Alumet pipeline.

For your first source, you will implement a simple counter that measures the number of times it has been triggered.

## Counter source: the idea

The counter source works in the following way.

It has an internal state: the current value of the counter.
The Alumet framework manages a timer and periodically triggers the source.
When triggered, the source creates a _measurement point_ containing the counter's value and appends it to a buffer provided by Alumet.
The framework then passes the data to the rest of the measurement pipeline.

![](../../resources/diagrams/plugin-tutorial/counter-source.png)

To distinguish the values of the counter from other unrelated measurements (such as the data produced by other sources), each _measurement point_ must indicate its _metric id_.
The _metric id_, represented by the rounded **(M)** on the diagram, indicates what "object" is being measured.
Your plugin must create a new metric on startup (see below), store its id somewhere, and use in to produce _measurement points_.

## Defining a metric

### What is a metric?

A metric represents an "object", something that can be measured. In Alumet, the measurement pipeline does not carry raw values but _measurement points_, which contain some additional information. Every point contains a _metric id_, which associate it with the definition of a _metric_.

Unlike some other tools, the list of metrics is not part of the framework: nothing is hard-coded. Plugins can create new metrics by following the standard model provided by Alumet. As explained [in the docs](https://docs.rs/alumet/latest/alumet/metrics), a metric is defined by:
- a **unique name**
- a **unit** of measurement (is it energy? time?) - Alumet follows the [UCUM (Unified Code for Units of Measure) standard](https://ucum.org/ucum).
- a **type** of measured value (is it an integer? a float?)
- a textual **description**

Read more about metrics here: [Metrics & Measurements Points](../metrics_measurements.md)

### Creating a metric

Okay, let's think about the metric that you need for your counter source.

- **name**: We'll call it `example_source_call_counter`, which reflects what we measure: we count how many times the source has been called by Alumet
- **unit**: Since this is a simple counter, it has no unit! In the UCUM, this corresponds to the "unity" unit.
- **type**: Integer, here we choose `u64` to keep it simple.
- **description**: Something short and explicit, for example `"number of times the example source has been called"`.

To register this new metric in Alumet, call `create_metric` in the `start` method of your plugin.

```rust,ignore 
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:plugin_start_head}}
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:create_source_metric}}
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:plugin_start_tail}}
```

If all goes well, you obtain a `TypedMetricId` that you can use to refer to the metric at every step of the Alumet pipeline, in particular in a source.

<div class="warning">

Creating a metric may fail in case of a duplicate name (note the `?` that handles the potential error - we'll talk about it later).

Therefore, you **should not** choose names that are too generic.
Instead, you **should** choose explicit and precise names.
If applicable, include some information about the kind of sensor.

Here are some examples:
- bad metric names (too vague): `metric`, `counter`, `measured_energy`
- good metric names: `acpi_zone_temperature`, `rapl_consumed_energy`, `estimated_gpu_power`, `kernel_cpu_usage`
</div>

## Defining a simple source

### Implementing the counter source

To define a source, define a structure and implement the `Source` trait on it (`alumet::pipeline::Source`).

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements_empty.rs:source}}
```

Complete the `poll` method by following these steps, and add the required fields along the way:
1. Measure, i.e. obtain the measurements.
    Here, we will simply increment a counter. The counter is part of the state of the source, hence it will be a field in the `ExampleSource` structure, of type `u64`.
2. Create one `MeasurementPoint` for every measured value.
    To do that, we need to know which metric we are measuring. The previously obtained metric id will be another field of the `ExampleSource` structure, of type `TypedMetricId`.
3. Push the points to the `MeasurementAccumulator`.

The result looks like this:
```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements_source_without_config.rs:source}}
```

Since our counter is not related to a particular _resource_ nor _consumer_, we use the special value `LocalMachine`.
It indicates that it's a "global" measurement, with the whole machine as a scope (here, "machine" is intentionally not precisely defined: if you're in a VM, it represents the VM, if you're running on a bare-metal node, it's this node).

Read more about the concept of _resource_ and _consumer_ here: [Metrics & Measurement Points](../metrics_measurements.md)

### Registering the counter source

Now that you have defined a source, you need to create it and add it to the Alumet pipeline with `add_source`.
Do this in the `start` method of your plugin.

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements_source_without_config.rs:plugin_start}}
```

Tip: you can click on the eye 👁️ icon to show the whole plugin code.

To add the source to the pipeline, it is required to provide a `Trigger`.
As its name implies, it will trigger the source in a certain way.
Here, we use `time_interval` to build a `Trigger` that will call the source every second.

## Final result

Finally, you can test your plugin again by running the local agent:
```sh
cargo run --bin alumet-local-agent --features local_x86
```

Note how the source is automatically shut down by Alumet when you stop the agent.

## A word about errors

In this chapter, we did not need to manage errors in a complicated way because there was almost no source of failure.
Most of our functions returned `Ok(())`, and we used `?` to propagate errors, for example in `start`.

Please refer to [Error Handling in Plugins](../error_handling.md) to learn how to handle errors in more complex cases.
