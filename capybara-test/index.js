const {ExportedClass} = require("./wasm_test");

let x = ExportedClass.add_and_print(21, 21);
console.log(x);
let instance = ExportedClass.new(42);
console.log(instance.get_number());