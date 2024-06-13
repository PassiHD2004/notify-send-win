# notify-send-win
## A notify-send replacement for Windows

### How to install:
- `cargo install notify-send-win`


### How to use:
- `notify-send-win`
- run `notify-send-win --help` for more information

```
Usage: notify-send-win.exe [OPTIONS]

Options:
  -a, --app <APP>                  "App Name" for the Toast [default: notify-send-win]
  -t, --title <TITLE>              "Title" for the Toast [default: "This is a default title"]
  -m, --message <MESSAGE>          "Message" for the Toast [default: "This is a default notification\nrun \"notify-send.exe --help\" for more information"]
  -l, --link <LINK>                Link to go to, when clicked [default: https://github.com/PassiHD2004/notify-send-win]
  -n, --notify-logo <NOTIFY_LOGO>  Logo for the notification [default: ]
  -i, --image <IMAGE>              Banner image to display [default: ]
  -d, --duration <DURATION>        How long to display the Toast. Options: "short" or "long" [default: short]
  -h, --help                       Print help
  -V, --version                    Print version
```
