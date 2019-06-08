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

### Display
You will see a randomly generated map in the browser console.

    壁壁壁壁壁壁壁壁壁壁壁壁壁
    壁　　　壁　　　　　　　壁
    壁　壁壁壁　壁壁壁　壁　壁
    壁　　　　　壁　壁　壁　壁
    壁　壁壁壁壁壁　壁壁壁壁壁
    壁　　　　　壁　壁　　　壁
    壁壁壁　壁　壁　壁壁壁　壁
    壁　壁　壁　　　壁　　　壁
    壁　壁　壁壁壁壁壁壁壁　壁
    壁　壁　　　　　　　　　壁
    壁　壁　壁　壁　壁壁壁　壁
    壁　　　壁　壁　　　壁　壁
    壁壁壁壁壁壁壁壁壁壁壁壁壁


