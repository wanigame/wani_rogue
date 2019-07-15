// Management of resource.
//
// Copyright (c) 2019 wanigame
//
// This source code is released under the MIT License
// http://opensource.org/licenses/mit-license.php

export class Resource {
    res_name = ["./res/mapchip.png"];

    image;
    load_flag;

    constructor() {
        this.image = [];
        this.load_flag = [];

        this.res_name.forEach(path => {
            let img = new Image();
            img.src = path;
            img.onload = this.load_complete.bind(this, path);

            this.image.push(img);
            this.load_flag.push(false);
        });
    }

    load_complete(path) {
        let index = this.res_name.indexOf(path);
        this.load_flag[index] = true;
    }

    get_image(index) {
        if (this.load_flag[index]) {
            return this.image[index];
        }
    }
}
