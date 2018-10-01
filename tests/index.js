const {ExportedClass} = require("./capybara_test");
const assert = require('assert');

assert(ExportedClass.add_and_print(21, 21) === 42);
assert((new ExportedClass(42)).get_number() === 42);

let instance = new ExportedClass(42);

ExportedClass.no_args();
ExportedClass.one_arg(42);
ExportedClass.two_args(42, 1337);

assert(ExportedClass.no_args_returning() === 42);
assert(ExportedClass.one_arg_returning(42) === 42);
assert(ExportedClass.two_args_returning(42, 1337) === 42);

instance.self_no_args();
instance.self_one_arg(42);
instance.self_two_args(42, 1337);

assert(instance.self_no_args_returning() === 42);
assert(instance.self_one_arg_returning(42) === 42);
assert(instance.self_two_args_returning(42, 1337) === 42);

instance.mut_self_no_args();
instance.mut_self_one_arg(42);
instance.mut_self_two_args(42, 1337);

assert(instance.mut_self_no_args_returning() === 42);
assert(instance.mut_self_one_arg_returning(42) === 42);
assert(instance.mut_self_two_args_returning(42, 1337) === 42);
