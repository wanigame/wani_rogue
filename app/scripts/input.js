// Receive input.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

export class Input {
    entry;

    constructor(entry) {
        this.entry = entry;

        document.onkeyup = this.key_up.bind(this);
        document.onkeydown = this.key_down.bind(this);
    }

    key_down() {
        if (this.entry.exports) {
            this.entry.exports.key_down(event.keyCode);
        }
    }
    key_up() {
        if (this.entry.exports) {
            this.entry.exports.key_up(event.keyCode);
        }
    }
}
