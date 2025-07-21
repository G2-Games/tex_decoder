# `.tex` file decoder
A very simple `.tex` file decoder from games such as NERTS! Online, The Zachtronics Solitaire Collection, Last Call BBS, Exapunks, etc.

The decoder simply takes in a list of paths to `.tex` files and outputs a file in the same location appended with `.png`. Conversion from `.png` to `.tex` is not supported (yet).

## Supported Versions
The version is specified in the first 4 bytes of the file.
- 1002 ✅
    - NERTS! Online
    - EXAPUNKS
- 1004 🟥 ([See here, needs implemented](https://gist.github.com/sigsegv-mvm/0f07b1c6d8dd56885f74e03758c11e58?permalink_comment_id=5031771#gistcomment-5031771))
    - Möbius Front '83
- 1005 ✅
    - Last Call BBS
    - Zachtronics Solitaire Collection

## Usage
```
tex_decode [files...]
```
