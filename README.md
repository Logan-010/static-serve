# static-serve
## this branch is the same as main, however it serves from the same directory as the executable rather than from site/
serves a static site on http://localhost:8080
NOTE!! this serves a index.html file located in the same directory as the executable file.
for example (windows), in the site's directory you would put serve.exe and index.html

file structure example
---
![file structure](https://raw.githubusercontent.com/Logan-010/static-serve/alternate/fs-example.png)

build via
```
cargo run
```
executable located at target/debug/serve.exe (windows)
