# Neo | E621 / E926 Image Downloader

## Changes from Arc-rust : Arc-rs

- No longer using Clap (Lower the size of the final binary file, and since there's only one command)
- Attempt to discontinue use of the Image lib

## Ideas

Same approach with Arc-rs but more refactored and stable.

Usage: neo.exe [config]
Usage (UNIX systems): neo [config]

Try to be as clean than Arc-rs' outputs (Should add Ansi Term and / or Loading and neofiglet)

Continue using YAML files as configuration files.
Possibly update caching method (Perhaps use JSON instead of TXT, proposal on why below)

JSON instead of TXT proposal:

## Dictionary

Keep a history of posts (Cache) and tags

### dictionary.json

```json
{
  "tags": ["tag1", "tag2"],
  "entries": { // Must be an array, because Rust :(
    "md5-id-here": {
      "ext": ".png"
    }
  }
}
```

So this equals

```json
{
  "tags": ["tag1", "tag2"],
  "entries": [{
    "md5": "md-5-id-here",
    "ext": ".png"
  }]
}
```
