# queens-cli

```
  /$$$$$$                                                            /$$$$$$  /$$       /$$$$$$
 /$$__  $$                                                          /$$__  $$| $$      |_  $$_/
| $$  \ $$ /$$   /$$  /$$$$$$   /$$$$$$  /$$$$$$$   /$$$$$$$       | $$  \__/| $$        | $$  
| $$  | $$| $$  | $$ /$$__  $$ /$$__  $$| $$__  $$ /$$_____//$$$$$$| $$      | $$        | $$  
| $$  | $$| $$  | $$| $$$$$$$$| $$$$$$$$| $$  \ $$|  $$$$$$|______/| $$      | $$        | $$  
| $$/$$ $$| $$  | $$| $$_____/| $$_____/| $$  | $$ \____  $$       | $$    $$| $$        | $$  
|  $$$$$$/|  $$$$$$/|  $$$$$$$|  $$$$$$$| $$  | $$ /$$$$$$$/       |  $$$$$$/| $$$$$$$$ /$$$$$$
 \____ $$$ \______/  \_______/ \_______/|__/  |__/|_______/         \______/ |________/|______/
      \__/                                                                                                                                                                                    
```
    
[![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)
![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)

![Showcase](https://raw.githubusercontent.com/kyncl/queens-cli/main/examples/example1.png)

Logical game inspired by Queens game from LinkedIn, but in CLI (please don't sue me, it's just project)

# Manual install
Requirements:
- Terminal
- Cargo

To build project you just need:
```bash
git clone git@github.com:kyncl/queens-cli.git
cd queens-cli
cargo build --release
# if you want to be able to call it anywhere
sudo cp ./target/release/queens-cli ~/.local/bin 
# or for all users
sudo cp ./target/release/queens-cli /usr/local/bin

# then just run
queens-cli -b <board-string>
```
Or just run:
```bash
cargo run --release -- -b <board-string>
```


# How to make board
Queens-cli has their own board format

For example:
```
6x6|\
0:0,0;1,0;2,0;2,1;\
1:0,1;0,2;0,3;1,3;\
2:3,0;1,1;2,2;3,1;1,2;3,2;\
3:4,0;5,0;4,1;4,2;2,3;3,3;4,3;\
4:5,1;5,2;5,3;5,4;\
5:0,4;1,4;2,4;3,4;4,4;0,5;1,5;2,5;3,5;4,5;5,5;\
|Q:|X:
```

If you want to run it:

```bash
queens-cli -b '6x6|0:0,0;1,0;2,0;2,1;1:0,1;0,2;0,3;1,3;2:3,0;1,1;2,2;3,1;1,2;3,2;3:4,0;5,0;4,1;4,2;2,3;3,3;4,3;4:5,1;5,2;5,3;5,4;5:0,4;1,4;2,4;3,4;4,4;0,5;1,5;2,5;3,5;4,5;5,5;| :|X:'
```

Every major value is separated by '|'
Values:
1. Board size (width x height)
2. Regions
    Each region is separated by number (like 0:) and all points that the region has
    Width and height is separated by `,` and points by `;`.
3. Queens
    If you for some reason want to put queens on some places. 
    The first value (the Q in example) is the skin that will be used for queens.
4. Empty places (Normally Xs)
    Same as Queens, but for empty places, where shouldn't be any queens

If you don't like CLI and want to play queens checkout [Queens](https://queensgame.vercel.app/) it's cool
