// Entry Point with Rust.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php


class Entry {
    exports;
    log = "";

    imports = {
        env: {
            js_log: function (log) {
                let c = String.fromCharCode(log);
                if (c == "\n") {
                    console.log(this.log);
                    this.log = "";
                }
                else this.log += c;
            }.bind(this),
            js_random: function (max) { return Math.floor(Math.random() * max); },
        }
    }

    constructor() {
        fetch("wani_rogue.wasm")
            .then((response) => response.arrayBuffer())
            .then((bytes) => WebAssembly.instantiate(bytes, this.imports))
            .then((results) => {
                this.exports = results.instance.exports;
                this.exports.init();
            });
    }
}

new Entry();
