# logger

Create log files (semi)-automatically. 

This is the tool I use to create a first version of the [coDual log](https://codual.github.io/log). It's probably mostly useful just to me.

## Configuration format

```yaml
timestamp_path: <path to timestamp file>
codual_path: <path to codual/codual.github.io repo folder>
frequency: weekly|monthly|bimonthly
wallabag:
    client_id: <Wallabag client ID>
    client_secret: <Wallabag client secret>
    username: <Wallabag username>
    password: <Wallabag password>
    base_url: <Wallabag instance URL, e.g. 'https://app.wallabag.it'>

```
