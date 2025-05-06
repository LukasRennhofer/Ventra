# Ventra

![License](https://img.shields.io/github/license/LukasRennhofer/Ventra)
![Version](https://img.shields.io/github/v/release/LukasRennhofer/Ventra)
![Issues](https://img.shields.io/github/issues/LukasRennhofer/Ventra)

Ventra is the official project and build manager of the Vantor Engine SDK

> [!IMPORTANT]
> Ventra is in early development and isnt fully stable yet
>

## Features

Ventra aims to create a convenient and easy to use development environment for developers using the Vantor Engine
Core concepts are based off of the Cargo package manager, such as short commands, modern terminal usage and a beginner friendly ecosystem.

## How do I run this?

> Ventra cannot be installed via releases, it needs to be built by yourself currently

See [instructions](TODO) for information on how to run Ventra.

> Ventra runs only on the Vantor supported dev platforms (Windows, Linux)

- Set up a new project:

```
ventra init project-name
```

- Build your project
    - Windows
        ```
        ventra build --release --platform Windows
        ```
    - Linux
        ```
        ventra build --release --platform Linux
        ```

    - Nintendo Switch (Homebrew)
        ```
        ventra build --release --platform SwitchHomebrew
        ```

## How do I read the documentation?

Command-related documentation can be found on the [vantor website](https://vantor.net).

## License

Ventra is licensed under the GNU General Public License v3, see **LICENSE** for more information.