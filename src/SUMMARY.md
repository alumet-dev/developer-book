# Summary

- [Introduction](./intro/Introduction.md)
- [Alumet Architecture](./intro/Alumet%20architecture.md)

# Developing plugins

- [Your first plugin (tutorial)](./plugins/tutorial/0_intro.md)
    - [Basic setup](./plugins/tutorial/1_start.md)
    - [Measuring with sources](./plugins/tutorial/2_measuring.md) <!-- todo a word about shutdown >
    - [Error handling](./plugins/tutorial/3_errors.md)
    - [Configuration management](./plugins/tutorial/4_config.md)
    - [Transforming measurements](./plugins/tutorial/5_transforms.md)
    - [Writing measurements](./plugins/tutorial/6_outputs.md)

- [Metric registration]() <!-- + best practices for naming metrics -->
- [What makes a measurement point?]() <!-- resource, resource consumer -->
- [Error handling for plugins]() <!-- anyhow -->
- [Configuration managment]() <!-- serde, toml -->
- [Shutdown]() <!-- pipeline elements are shutdown before stop() is called -->
- [Gathering data with measurement sources]()
    - [Two kinds of source]() <!-- managed vs autonomous -->
    - [Adding sources during startup]() <!-- add_source, config for Tigger -->
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
