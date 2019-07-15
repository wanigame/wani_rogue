// Painter for canvas.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

import { Resource } from "./resource.js";

export class Painter {
    canvas;
    context;

    resource;

    constructor() {
        this.canvas = document.getElementById("main");
        this.context = this.canvas.getContext("2d");

        this.resource = new Resource();
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

    draw_image(index, sx, sy, sw, sh, dx, dy, dw, dh) {
        let img = this.resource.get_image(index);
        if (img) {
            this.context.drawImage(img, sx, sy, sw, sh, dx, dy, dw, dh);
        }
    }
}
