# WC-RUST

I made this simple project as a way to start learning rust, i wanted to do some basic cli tool and i went with linux wc, in this program i didn't implement counts on bytes, therefor there is not `-m` flag and the `-c` flag always count only characters. This program uses rayon library for parallelism operations like count, split, lines, ecc...

The program is fast but not efficent on resource consumption, at the time if you pass a file of e.g. 2GB the program allocates the 2GB file in the memory becouse `read_as_string` is used for file opening (not efficent), maybe i'll re-write the code to support file streams.

the command takes input from files or from stdin by piping to it, flags and files can be placed everywhere in the arguments (there is not order) the ony thing important is to use the `-` for commands (they can be joined in a single string for example `-Lwc`.

This is a list of command examples
```
./wcrust file.txt
./wcrust file.txt -w
./wcrust -w file.txt
./wcrust file1.txt file2.txt -w -c -l
./wcrust file1.txt file2.txt -wcl
./wcrust file1.txt file2.txt -L
```

for the porpouse of the flags i refer you to the official (man page)[https://man7.org/linux/man-pages/man1/wc.1p.html] of `wc` 

pull request are kindly accepted☺️

PS: I hope everything is written in a fairly correct english
