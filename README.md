# Downloader
Multithread downloads at glance

```sh
  brew install rawnly/tap/download
```

## Usage

```
  # Basic usage
  downloader <url1> <url2> -o output_dir


  # Advanced
  # Example downloading raycast wallpapers

  mkdir -p raycast_wallpapers
  curl https://www.raycast.com/wallpapers.json | jq ".[].url" | xargs downloader -o raycast_wallpapers
```


