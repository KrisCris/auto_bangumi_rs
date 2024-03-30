# auto_bangumi_rs

A simple bangumi renamer, for now...

You can run it standalone or probably have it triggered by other applications.

## qBittorrent run external program on completion: 
```bash
# If you wish the tool grouping files based on series and seasons
# They will be grouped by folders, i.e. /Path/To/All/Your/Animes/SeriesName/Season Number/Renamed File Name.suffix
auto_bangumi_cli -i "%F" -o "/Path/To/All/Your/Animes" -g move

# Or, just rename inplace:
auto_bangumi_cli -i "%F" -o "%D" move
```

## Standalone:

https://github.com/KrisCris/auto_bangumi_rs/assets/38860226/19bdd02c-f69d-4cc2-9f40-afd1c91f8aec




## Credit
- EstrellaXD for creating [Auto_Bangumi](https://github.com/EstrellaXD/Auto_Bangumi).


