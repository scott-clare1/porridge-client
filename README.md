# porridge-client

A rust client with python bindings for the [porridge](https://github.com/scott-clare1/porridge) vector search engine.

## Installation
Currently to use the python client you will need to clone the repository and then compile the rust code into python with `maturin` which installs the libary directly into a virtual environment.

If you are using Poetry then add `maturin` like this:
```bash
poetry add -G dev maturin
```

Now clone the repo and build the crate - install the client directly onto the virtual environment.

```bash
git clone git@github.com:scott-clare1/porridge-client.git
maturin develop
```

## How it works
The clients methods map directly to the endpoints on the server - see the [README.md](https://github.com/scott-clare1/porridge/blob/master/README.md) for more info.

## Roadmap
Publish to PyPi so that the client can be installed directly with `pip` or added as a dependency to a `pyproject.toml` file.
