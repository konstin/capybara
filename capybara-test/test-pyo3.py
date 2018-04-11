#!/usr/bin/env python3

import json
import os
import shlex
import subprocess
from importlib import import_module
from shutil import copyfile


def run_structured_tests(ExportedClass):
    instance = ExportedClass(42)

    ExportedClass.no_args()
    ExportedClass.one_arg(42)
    ExportedClass.two_args(42, 1337)

    assert ExportedClass.no_args_returning() == 42
    assert ExportedClass.one_arg_returning(42) == 42
    assert ExportedClass.two_args_returning(42, 1337) == 42

    instance.self_no_args()
    instance.self_one_arg(42)
    instance.self_two_args(42, 1337)

    assert instance.self_no_args_returning() == 42
    assert instance.self_one_arg_returning(42) == 42
    assert instance.self_two_args_returning(42, 1337) == 42

    instance.mut_self_no_args()
    instance.mut_self_one_arg(42)
    instance.mut_self_two_args(42, 1337)

    assert instance.mut_self_no_args_returning() == 42
    assert instance.mut_self_one_arg_returning(42) == 42
    assert instance.mut_self_two_args_returning(42, 1337) == 42


def main():
    subprocess.check_call(shlex.split("cargo build --features capybara-python"))

    metadata = json.loads(subprocess.check_output(shlex.split("cargo metadata --format-version 1")))
    python_module_name = metadata["resolve"]["root"].split(" ")[0].replace("-", "_")
    python_so_name = python_module_name + ".so"
    cargo_lib_name = "lib" + python_module_name + ".so"
    path = os.path.join(metadata["target_directory"], "debug", cargo_lib_name)
    copyfile(path, python_so_name)

    capybara_test = import_module(python_module_name)

    added = capybara_test.ExportedClass.add_and_print(21, 21)
    assert added == 42
    assert 42 == capybara_test.ExportedClass(42).get_number()

    run_structured_tests(capybara_test.ExportedClass)

    os.remove(python_so_name)


if __name__ == '__main__':
    main()
