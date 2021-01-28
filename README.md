qmc-decode
==========

A lightweight command line tool to decode `qmc` file into `mp3`, `flac` or `ogg`. It offers more flexibilities than the 
[original version](https://github.com/jixunmoe/qmc-decode) written in C. Speedwise, the C version and this rust version
is very similar -- they are both very fast. However,the rust version has much bigger build size. On macOS, it is about
791K after being stripped, while the C version is about 49K. I did not see much a difference stripping the C version --
it is tiny. Having said that, 791K is also tiny given that much more libraries are used.

Here is the command line help:

```
target/release/qmc-decode -h                                                                                                                        
qmc-decode 0.1.0
Hongze Xia <hongzex@gmail.com>
Encoding/decoding QMC files
    if the extension ends with `qmc0` or `qmc3`, convert it to `mp3`
    if the extension ends with `qmcflac`, convert it to `flac`
    if the extension ends with `qmcogg`, convert it to `ogg`

USAGE:
    qmc-decode [OPTIONS] <INPUT>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --buffer-size <BUFFER_SIZE>    buffer size for reading files [env: BUFFER_SIZE=]  [default: 1048576]
    -o, --outdir <OUTDIR>              output directory [default: .]

ARGS:
    <INPUT>...    list of files or glob expressions to convert
```
