const {MyClass} = require("./wasm_test");

let x = MyClass.add_and_print(21, 21);
console.log(x);
let instance = MyClass.new(42);
console.log(instance.get_number());