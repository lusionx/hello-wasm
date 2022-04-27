const js = import("../pkg/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly aa");
  console.log(js.add2(1, 2));
  let s = `<a href="/404.html">404</a>`;
  console.log(js.queryAttr(s, 'a', 'href'))
});