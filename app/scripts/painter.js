// Painter for canvas.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

export class Painter {
    canvas;
    context;

    constructor() {
        this.canvas = document.getElementById("main");
        this.context = this.canvas.getContext("2d");
    }

    draw_rect(x, y, width, height, color) {
        this.context.fillStyle = color;
        this.context.fillRect(x, y, width, height);
    }

    clear_rect() {
        this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
    }

    color(r, g, b, a = 0xff) {
        return "rgba(" + r + "," + g + "," + b + "," + a / 0xff + ")";
    }
}
