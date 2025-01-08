# Configuration Management

The counter source that you have implemented in the previous section uses a fixed polling interval.
Instead of using a hard-coded value, it would be better to provide a configuration option so that we can choose the acquisition frequency of the source before starting the Alumet agent.

## Configurable polling interval: the idea

## Implementation

### Required dependency

To serialize and deserialize the configuration, Alumet uses `serde`, which is the de-facto standard framework for (de)serialization.
Add it to the dependencies by running this command in the plugin's directory:
```sh
cargo add serde --features derive
```

### Config structure

### Default config

### Config loading

