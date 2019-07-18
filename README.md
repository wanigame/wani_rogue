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
![github](https://user-images.githubusercontent.com/51026231/61468828-1e113980-a9b9-11e9-8b6b-975c74e4d008.gif)
