Text transformer bot for Telegram
---------------------------------

A simple Telegram bot for transforming text in different manners, rewritten in Rust. Ever felt the need to  e m p h a s i z e  something with more spaces? Or perhaps you take it eeeeeeeeeasy? Wanna MoCk SoMeOnE? I do the work of pressing Space and Shift for you.

The bot is hosted at [@txtorbot](https://t.me/txtorbot). If you want to host it yourself, install the stable version of Rust compiler (2018 edition), compile the project using `cargo build`, pass the `TELOXIDE_TOKEN` environment variable and run the program.

To-do:
* Add more tests for `truncate` (for example, I'm sure it'll break with CJK characters on the truncation boundary)

#### License

See [UNLICENSE](/UNLICENSE).
