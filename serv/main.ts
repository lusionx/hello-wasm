import * as wasm from '../pkg/hello_wasm'

function main() {
    let s = `<a href="/404.html">404</a>`;
    console.log(wasm.queryAttr(s, 'a', 'href'))
}

process.nextTick(main)