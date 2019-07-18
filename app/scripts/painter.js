// Painter for canvas.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

import { Resource } from "./resource.js";

export class Painter {
    main_canvas;
    main_context;

    buf_canvas;
    buf_context;

    resource;

    constructor() {
        this.main_canvas = document.getElementById("main");
        this.main_context = this.main_canvas.getContext("2d");

        this.buf_canvas = document.getElementById("buffer");
        this.buf_context = this.buf_canvas.getContext("2d");

        this.resource = new Resource();
    }

    flip() {
        let img = this.buf_context.getImageData(0, 0, this.buf_canvas.width, this.buf_canvas.height);
        this.main_context.putImageData(img, 0, 0);
    }

    draw_rect(x, y, width, height, color) {
        this.buf_context.fillStyle = color;
        this.buf_context.fillRect(x, y, width, height);
    }

    clear_rect() {
        this.buf_context.clearRect(0, 0, this.buf_canvas.width, this.buf_canvas.height);
    }

    color(r, g, b, a = 0xff) {
        return "rgba(" + r + "," + g + "," + b + "," + a / 0xff + ")";
    }

    draw_image(index, sx, sy, sw, sh, dx, dy, dw, dh) {
        let img = this.resource.get_image(index);
        if (img) {
            this.buf_context.drawImage(img, sx, sy, sw, sh, dx, dy, dw, dh);
        }
    }
}
