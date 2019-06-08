// Entry Point with Rust.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

let _exports;
let _log = "";

fetch("wani_rogue.wasm")
    .then((response) => response.arrayBuffer())
    .then((bytes) => WebAssembly.instantiate(bytes, imports))
    .then((results) => {
        _exports = results.instance.exports;
        _exports.init();
    });

const imports = {
    env: {
        js_log: function (log) {
            let c = String.fromCharCode(log);
            if (c == "\n") {
                console.log(_log);
                _log = "";
            }
            else _log = _log + c;
        },
        js_random: function (max) { return Math.floor(Math.random() * max); },
    }
}
