# Stwitch

stwitch(stream twitch) is a cli tool to stream live or vod's of your favorite twitch streamers

## Usage
  ### How it works
  run stwitch <query> to get a selection of streams to choose from by the streamer you passed as a query. If the streamer is currently live the top link will indicate so by being prefixed with `LIVE: `. The chosen selection will then be passed through to mpv to watch.
    

## Flags

  - -s | --subscribe <query>: when no query is added you can choose from a list of streamers you already `subscribe` to. If a query is inputted that query will be added to your "Subscribe" list and the program will continue
  - -d | --detach: detaches the player from the terminal 

## Configs

The config file will be created depending on what system you are running. for MacOS it is `$HOME/Library/Preferences/rs.stwitch/stwitch.toml`.

The config file will need to have the following:

```
  client_id = '{YOUR CLIENT ID}'
  secret = '{YOUR SECRET KEY}'
  subscribes = [{ARRAY OF TWITCH STREAMER NAMES}]
```


## Dependencies

* mpv

