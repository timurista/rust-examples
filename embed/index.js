var ffi = require("ffi");

var lib = ffi.Library("target/release/libembed", {
  fibonacci: ["int", ["int"]]
});

console.time("fib");
console.log(lib.fibonacci(38)); // 63245986
console.timeEnd("fib");
