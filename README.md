# wallhaven-dl

Checks your firefox tabs, downloads wallhaven wallpapers with a certain resolution.

Removes the downloaded wallpapers' firefox tabs (if there is any error, tells you the wallpaper id).

Nothing more, nothing less. 

## Installation

As simple as (Make sure to edit the `<repo-url>` AND to install `cargo`): 
```sh
git clone <repo-url> && cd wallhaven-dl && cargo install --path .
```

## Help

Here's the help message:

```
Usage: wallhaven-dl [OPTIONS]

Options:
  -o, --outdir <OUTDIR>          Directory to which write the images [default: ~/Pictures/DownloadedWallpapers]
  -a, --api-key <API_KEY>        The api key that needs to be used. If not present, tries using WALLHAVEN_API_KEY, or else defaults to none
  -r, --resolution <RESOLUTION>  Image output resolution. Your problem to know which ones are possible, not mine tbh [default: 1920x1080]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Misc

This has been written with 🧡 and lazyness in Rust.


Sorry if the README is not the best one.


PRs are welcome, please feel free to help!

### License

You can do whatever you want with this