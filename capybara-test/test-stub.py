#!/usr/bin/env python3
"""
Checks that the stub target compiles and does not emitt unexpected symbols
"""

import json
import os
import shlex
import subprocess


def main():
    subprocess.check_call(shlex.split("cargo build"))

    metadata = json.loads(subprocess.check_output(shlex.split("cargo metadata --format-version 1")))
    python_module_name = metadata["resolve"]["root"].split(" ")[0].replace("-", "_")
    cargo_lib_name = "lib" + python_module_name + ".so"
    path = os.path.join(metadata["target_directory"], "debug", cargo_lib_name)

    symbols = subprocess.check_output(shlex.split("readelf -Ws") + [path])
    symbols = symbols.decode("utf-8").split("\n")
    symbols = [x for x in symbols if " _" not in x]
    symbols = [x.split(" ")[-1] for x in symbols]
    assert "rust_eh_personality" in symbols  # That's in every rust lib
    assert "PyErr_Print" not in symbols  # That would be pyo3
    assert "Init_native" not in symbols  # That would be helix


if __name__ == '__main__':
    main()
