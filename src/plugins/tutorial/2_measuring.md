# Measuring with sources

For the moment, your plugin doesn't measure anything.
You will now write your first "source" in order to obtain measurement points, that will be passed to the rest of the Alumet pipeline.

For your first source, you will implement a simple counter that measures the number of times it has been polled.

## Defining a metric

### What is a metric?

Before defining a source, you need to know what type of "object" it will measure, what kind of information it will obtain.
This information is defined in a _metric_. Alumet offers a standard way of defining metrics, in a way that makes all the measurements useful and rigorous.

As explained [in the docs](https://docs.rs/alumet/latest/alumet/measurement), a metric is defined by:
- a _unique name_
- a _unit_ of measurement (is it energy? time?) - Alumet follows the [UCUM standard](https://ucum.org/ucum) for units.
- a _type_ of measured value (is it an integer? a float?)
- a textual _description_

For efficiency reasons, Alumet assigns a unique id to every metric, and uses this id instead of the full definition or name.

Okay, let's think about the metric that you need for your counter source.

- **name**: We'll call it `example_source_call_counter`, which reflects what we measure
- **unit**: Since this is a simple counter, it has no unit! This corresponds to the "unity" unit.
- **type**: Integer, `u64`.
- **description**: Something short and explicit, for example `"number of times the example source has been called"`

### Registering the metric

To register this new metric in Alumet, add the following lines to the `start` method.

```rust,ignore 
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:create_source_metric}}
```

Creating a metric may fail in case of a duplicate name. Therefore, you should **not** choose names that are too generic (for instance, avoid naming a metric `metric`, `counter` or `measured_energy`). If all goes well, you obtain a `TypedMetricId` that you can use to refer to the metric at every step of the Alumet pipeline, in particular in a source.

## Defining a simple source

To define a source, define a structure and implement the `Source` trait on it (`alumet::pipeline::Source`).

The core of Alumet will automatically call the `poll` method when it is time to do so.
Each time a source is triggered (that is, its `poll` method is called), it can produce new measurement points and push them to the accumulator.

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:source_partial0}}
    // TODO
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:source_partial1}}
        todo!()
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:source_partial2}}
```

To implement `poll`, follow these steps:
1. Measure, i.e. obtain the measurements.
    Here, we will simply increment a counter. The counter is part of the state of the source, hence it will be a field in the `ExampleSource` structure.
2. Create one `MeasurementPoint` for every measured value.
    To do that, we need to know which metric we are measuring. The previously obtained `TypedMetricId` will be another field of the `ExampleSource` structure.
3. Push the points to the `MeasurementAccumulator`.

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:source}}
```

What are the _resource_ and _resource consumer_, you may ask?

Well, Alumet has been created to measure energy consumption and performance-related metrics. To unify the metadata and improve the way that we can consume the measurements in models, we have added these two fields. A _resource_ is something that can be "consumed", usually a piece of hardware. For example, CPU cores and RAM are resources to Alumet. A _consumer_ is something that uses a resource. It can be hardware or software. For example, a process is a resource consumer to Alumet.

Together, resources and resource consumers make the perimeter of the measurement more precise. They also help avoiding an explosion of the number of metrics.

A typical example is the case of RAPL measurements.
The `rapl` source for Alumet defines a metric `rapl_consumed_energy`, in Joules, and uses it for all RAPL measurements, even if their perimeter is different: CPU package, CPU cores, RAM, etc. The metric is the same, but the resource is different.

To come back to our tutorial, here we use `LocalMachine` to indicate that it's a "global" measurement, not limited to a specific piece of hardware nor to a specific software component.

## Registering the source

Now that you have defined a source, you need to add it to the Alumet pipeline.
To do so, add the following lines to the `start` method of your plugin:

```rust,ignore
{{#rustdoc_include ../../../code/plugin_example/src/basic_with_elements.rs:add_source}}
```
