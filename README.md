# logger

Create log files (semi)-automatically. 

This is the tool I use to create a first version of the [coDual log](https://codual.github.io/log). It's probably mostly useful just to me.

## Configuration format

```yaml
codual_path: <path to codual/codual.github.io repo folder>
frequency: weekly|monthly|bimonthly
karakeep:
    api_key: <Karakeep API key (generate in Settings > API Keys)>
    base_url: <Karakeep instance URL, e.g. 'https://karakeep.example.com'>
    list_id: <ID of the Karakeep list to fetch bookmarks from>

```
