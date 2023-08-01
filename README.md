# rs-tcp-server

this is a little tcp server that I made. I tried to make it without looking at the book too much. Its single threaded and only supports plaintext filtypes like html and css but its pretty neat.

It supports file based routing (with subdirectories!) but has many drawbacks. look at `./www/index.html` for all the bullet points

you can find all the example html files in `./www/`.

you can find the rust code in `./src/main.rs`.

there are a few things you can configure in the `main.rs` file such as the path for all your html documents or which port to run on by configuring the constants at the top of the file.