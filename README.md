# wani_rogue
This is RogueLike system by WebAssembly with Rust.

## How to use
Run the following command to ganerate .wasm file.

    $ cargo build --target=wasm32-unknown-unknown
And, incorporate <em>www/entry.js</em> into HTML file.

e.g.)

    <!DOCTYPE html>
    <html lang="ja">

    <head>
        <meta charset="utf-8">
    </head>

    <body>
        <script src="./entry.js" type="module"></script>
    </body>

    </html>
Place these file on your server.

## Example
[https://wanigame.com/rogue/](https://wanigame.com/rogue/)
You can see a randomly generated map and operate "WASD" or cursor key.

### Display
![screenshot](https://user-images.githubusercontent.com/51026231/60900085-946db780-a2a6-11e9-9f32-70f6044ea16f.gif)
