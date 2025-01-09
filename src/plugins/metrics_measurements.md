# Metrics & Measurement Points

## Measurement points

Alumet can manage a large number of measurements that come from different sources implemented in multiple plugins.
Its measurement pipeline does not carry raw values, but _measurement points_ which contain some metadata in addition to the raw value.

The following diagram represents the data contained in a point, and its relation with metrics. It is explained in details in the next sections.

![](../../resources/diagrams/alumet-measurement-points.png)

## Metrics

In particular, we want to know what "object" is being measured, what kind of information is obtained: is it the temperature of an ACPI sensor? The energy consumed by the CPU? The memory reserved by a process? What is the associated unit? This knowledge is stored in a _metric_.

Alumet offers a standard way of defining metrics, in a way that makes all the measurements useful and rigorous.
As explained [in the docs](https://docs.rs/alumet/latest/alumet/metrics), a metric is defined by:
- a **unique name**
- a **unit** of measurement (is it energy? time?) - Alumet follows the [UCUM (Unified Code for Units of Measure) standard](https://ucum.org/ucum).
- a **type** of measured value (is it an integer? a float?)
- a textual **description**

For efficiency reasons, Alumet assigns a unique id to every metric, and uses this id instead of the full definition or name.
Each _measurement point_ hence contains a _metric id_.

## More than metrics: resources and resource consumers

Sometimes, we need a scope that is more precise than the metric definition.
For instance, when measuring the use of the CPU by the OS kernel, we are interested in knowing the value per CPU core.

This could be implemented by creating one metric for each case: `kernel_cpu_usage_core0`, `kernel_cpu_usage_core1`, ...
Some monitoring tools use this strategy.
However, it is is too limiting: it complicates the operations that you can apply on the data (think of filters, aggregates, etc.), and it does not scale well when you have multidimensional information to add to the measurements (CPU core id, hostname, etc.).

Therefore, we have chosen a different model for Alumet.
First, arbitrary key-value pairs can be attached to a measurement point. We call them _attributes_.
Second, two common pieces of information are always present in a measurement point: the _resource_ and _resource consumer_.

- An **attribute** is a key-value pair that can be attached to a measurement point. Its content is completely arbitrary.
- A **resource** is something that can be "utilized" or "consumed". It is usually related to a piece of hardware. For example, CPU cores and RAM are _resources_ to Alumet.
- A **consumer** is something that uses a resource. It is usually a software component. For example, a process is a _resource consumer_ to Alumet.

Attributes are optional, but the resource and consumer fields are mandatory.

<div class="warning">

Since Alumet supports multidimensional data, no "dimension" should not appear in metric names. Furthermore, the preferred word separator is `_`.
- bad metric names: `kernel_cpu_usage_core0`, `rapl.energy.NODE-A123`, `nvidia-gpu-[00:08.0]-estimated_power`
- good metric names: `kernel_cpu_usage`, `rapl_consumed_energy`, `estimated_gpu_power`
</div>
