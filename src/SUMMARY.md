# Summary

- [Introduction](./intro/Introduction.md)
- [Alumet Architecture](./intro/Alumet%20architecture.md)

# Developing plugins

- [Your first plugin (tutorial)](./plugins/tutorial/0_intro.md)
    - [Basic setup](./plugins/tutorial/1_start.md)
    - [Measuring with sources](./plugins/tutorial/2_measuring.md)
    - [A First Configuration](./plugins/tutorial/3_config.md)
    - [Transforming measurements](./plugins/tutorial/4_transforms.md)
    - [Writing with outputs](./plugins/tutorial/5_outputs.md)

- [Metrics & Measurement Points](./plugins/metrics_measurements.md)
- [Error Handling for Plugins](./plugins/error_handling.md)
- [Configuration management]() <!-- serde, toml -->
- [Shutdown]() <!-- pipeline elements are shutdown before stop() is called -->
- [Gathering data with measurement sources]()
    - [Two kinds of source]() <!-- managed vs autonomous -->
    - [Adding sources during startup]() <!-- add_source, config for Trigger -->
    - [Adding sources later]() <!-- ControlHandle -->
- [Processing with transform functions]() <!-- ?? -->
- [Exporting data with outputs]()
    - [Two kinds of output]() <!-- blocking vs async -->
- [Pipeline control]() <!-- on-the-fly pipeline reconfiguration -->

# Contributing to Alumet

- [The repositories]()
- [Reporting an issue or asking a question]()
- [Alumet crates]()
- [Alumet architecture]()
    - [High-level introduction]()
    - [The measurement pipeline]()
    - [Source tasks]()
    - [Transform tasks]()
    - [Output tasks]()
    <!-- insist on the difference between tasks (in alumet) and elements (in plugins) -->
- [Creating a plugin in the alumet repository]()
- [Moving a plugin to the alumet repository]()
