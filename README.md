# plugin-manager-launcher-rs
CLI module for Plugin Manager for Rust language

## Description
This is first module of plugin manager that can be used as core framework for building modular multi-threaded applications.

## Motivation
Basic idea for this component is to simplify development of complex solution, by splitting it into smaller modules, while complete solution will be represented as set of modules that will be interacting with each other through interfaces. This approach can be used as well as for Desktop, Web, CLI or Game Dev, that consists of modules that could be separated from each other and be developed independently. 

The finite goal for the plugin manager, is to give ability for the end user application to configure neccessary module in runtime or compile-time, but leaving core design of that end user application unchaged.

## Features:
- **Compile-Time Configuration** - bootstraiping multi-threaded application by adding core plugins into plugin manager during application start-up [**work in progress**]
- **Dynamic Libraries Modular load** - be able to load external dynamic libraries as plugins: dll/so/dylib [**work in progress**]
- **Command Line Arguments** - pareamtrized external dynamic libraries to load, during application startup [**planned feature**]
- **CLI as Interactive mode** - dynamic load/unload of plugins in runtime [**planned feature**]
- **GUI based plugin Loader** - not planned, but should be possible to implement as external plugin [**optional**]

## Status
**Tasks and Plans:**
1. Core design of component is still under development.
2. Short-term goal is to split this component into two crates: **plugin-manager-api-rs** and **plugin-manager-launcher-rs**.
3. Write FAQ about ABI Compatibility
4. Write User Guide document

## License: 
This component will be distributed under two licenses **copyleft(LGPL)** and **permissive(Apache)**.

It is under our interest that final product can be used as a part of commercial close-source software, however we are also interesting about sharing improvement for this framework.

**Commerical Use:** The API part is distributed with permissive licnse, so you can satically link Plugin Manager API into you code, and distributed you business plugins as closed-source software library that will work together with open-source launcher.
**Open Source Use:** We are intersted about sharing of improvements for plugin manager, so any changes to plugin manager core framework should be shared to Open Source community accoring to requirement of copyleft license above.

Plugin Manager core design that Plugin Manager also can be improved using plugin system, and according to license rules, we can not disallow distribution of such module under closed-source use, however we would recommend to share any plugins with open-source that improves core design and related to core goals of plugin manager, since you could get user support from community.

## Contribution
- If you have any ideas, question or suggestion, feel free to ask questions using contact bellow.
- If you would like to help project by creating pull reuqest, we would be glad to assist you in any possible way.
  - Project goals are described above
  - Please do not hesitate to write mail with your suggestion, before creating pull request.

## Contacts
Dmytro B - amid.ukr@gmail.com
