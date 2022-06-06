# Wordle #

This is a small **Rust** project which creates an **aided solver** for the Wordle game.
The compiled programme when run provides updated optimal ‘next guess‘ suggestions.
Note that this implementation is currently suited for the [New York Times](https://www.nytimes.com/games/wordle/index.html) version. Technically will work with any other implementation in any language, provided a sufficiently large word list is added to the project.

I first created a prototype in python, which is stored in the folder [./src-py/](src-py/).

## System Requirements ##

### Miscellaneous ###

Developers should ideally have the following installed:

- a **bash** terminal. This is already provided with Linux/OSX.
  Windows users can install bash for example via <https://gitforwindows.org> (-> gitbash).
- the **just** tool, which can be found on [here](https://github.com/casey/just#installation).

### Rust ###

The current project has been developed so far with the cargo package manager **v1.56.0**.
Rust can be installed via **rustup** from [here](https://www.rust-lang.org/tools/install),
which installs both Rust and cargo
(cf [documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html)).

### Python (optional) ###

Use **Python 3.10.x** (only relevant if one wishes to use the python code).

## Usage ##

### Setup ###

Provided the above system requirements are satisfied, run
```bash
just setup
```
from the root of the project.

Provided setup has been completed and system requirements are met, call:
```bash
just build
```
in a bash terminal from the project root,
in order to install the package dependencies ('crates').

### Execution ###

In a bash terminal from the project root, call
```bash
just build # only need this once
just run
```
The `build` target creates a binary in the [`./dist/`](dist/) folder,
and this can also be manually executed.

Since upon building the assets are embedded, this binary artefact can in fact be
moved anywhere in your system and be executed without requiring this project folder.

## Examples ##

See [./examples/](examples/).

## Limitations ##

The word list ([./assets/words.txt](assets/words.txt)) have been extracted from the source
code of the [**New York Times** website](https://www.nytimes.com/games/wordle/index.html).
The app should work with any list of (English) words, provided this are sufficiently exhaustive
(in particular, they should contain at least all---currently 2309---words that NYT use).

Due to the optimisation strategie employed in this app,
technically the aided solver operates in ‘hard mode‘.

## Future / Todos ##

- unit tests.
- more complete rust documentation.
- end-to-end tests.
- examples.
- dynamical loading of word list e.g. from an online source.
- a GUI (e.g. via imgui) which allows the user to switch word lists and choose tactics.
