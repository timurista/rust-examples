var ffi = require("ffi");

var lib = ffi.Library("target/release/libembed", {
  fibonacci: ["int", ["int"]]
});

console.log(lib.fibonacci(13)); // 89
