// Entry Point with Rust.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

import { Input } from "./input.js";
import { Painter } from "./painter.js";

class Entry {
    exports;
    log = "";

    painter = new Painter();

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
            js_random: function (max) {
                return Math.floor(Math.random() * max);
            },
            js_draw_rect: function (x, y, w, h, r, g, b, a) {
                this.painter.draw_rect(x, y, w, h, this.painter.color(r, g, b, a));
            }.bind(this),
        }
    }

    constructor() {
        fetch("wani_rogue.wasm")
            .then((response) => response.arrayBuffer())
            .then((bytes) => WebAssembly.instantiate(bytes, this.imports))
            .then((results) => {
                this.exports = results.instance.exports;
                this.exports.init();

                setInterval(() => {
                    this.exports.update();
                    this.painter.clear_rect();
                    this.exports.draw();
                }, 20); // 50FPS
            });
    }
}

window.onload = () => {
    const entry = new Entry();
    new Input(entry);
};
