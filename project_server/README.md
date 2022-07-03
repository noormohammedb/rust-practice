### for auto reload

- install entr cli utility

> `sudo apt install entr`

- run for auto restart on file change

> `ls src/\*\*/\*.rs | entr -r -s "cargo run"`
