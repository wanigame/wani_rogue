// Painter for canvas.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

export class Painter {
    context;

    constructor() {
        this.context = document.getElementById("main").getContext("2d");
    }

    draw_rect(x, y, width, height, color) {
        this.context.fillStyle = color;
        this.context.fillRect(x, y, width, height);
    }

    color(r, g, b, a = 0xff) {
        return "rgba(" + r + "," + g + "," + b + "," + a / 0xff + ")";
    }
}
