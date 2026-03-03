# beemorph

- morphometric search for bees
- interactive user interface to the global diversity data.
- Download the interaction and place them in the interaction folder. 

```
_                                                        _     
| |__     ___    ___   _ __ ___     ___    _ __   _ __   | |__  
| '_ \   / _ \  / _ \ | '_ ` _ \   / _ \  | '__| | '_ \  | '_ \ 
| |_) | |  __/ |  __/ | | | | | | | (_) | | |    | |_) | | | | |
|_.__/   \___|  \___| |_| |_| |_|  \___/  |_|    | .__/  |_| |_|
                                                |_|            

Interaction portal for bee interaction
     ************************************************
     Gaurav Sablok,
     Email: codeprog@icloud.com
    ************************************************

Usage: beemorph <COMMAND>

Commands:
interactive         browse the interactions
diversity-database  make a database
help                Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version


```

```
_                                                        _     
| |__     ___    ___   _ __ ___     ___    _ __   _ __   | |__  
| '_ \   / _ \  / _ \ | '_ ` _ \   / _ \  | '__| | '_ \  | '_ \ 
| |_) | |  __/ |  __/ | | | | | | | (_) | | |    | |_) | | | | |
|_.__/   \___|  \___| |_| |_| |_|  \___/  |_|    | .__/  |_| |_|
                                                |_|            

browse the interactions

Usage: beemorph interactive <FILEPATH> <THREAD> [TAXONID] [TAXONTDS] [TAXONNAME] [TAXONPATHNAME] [TAXONPATHID] [TAXONSUBGENUSID] [TAXONGENUSNAME] [TAXONFAMILYID] [TAXONFAMILYNAME] [WILDSEARCH]

Arguments:
<FILEPATH>         path to the file
<THREAD>           threads for the analysis
[TAXONID]          search by taxonid
[TAXONTDS]         search by taxontds
[TAXONNAME]        search by taxonname
[TAXONPATHNAME]    search by taxonpath
[TAXONPATHID]      search by taxon path id
[TAXONSUBGENUSID]  search by the taxon sub genus id
[TAXONGENUSNAME]   search by the taxon genus name
[TAXONFAMILYID]    search by taxon family id
[TAXONFAMILYNAME]  search by taxon family name
[WILDSEARCH]       search wild categoris

Options:
-h, --help  Print help

```

- sample field names 
```
sourcetaxonid: 1131416	
sourcetaxontds: 1131416	
sourcetaxonname: Pompiloides	
sourcetaxonpathnames: Animalia | Arthropoda | Insecta | Hymenoptera | Pompiloides (Wild search, you can give any of them)	
sourcetaxonpathids: IRMNG:11 | IRMNG:235 | IRMNG:1096 | IRMNG:10830 | IRMNG:1131416 (Wild search, you can give any of them)	
sourcetaxonsubgenusid: 10830	
sourcetaxongenusname: Insecta	
sourcetaxonfamilyid:IRMNG:235	
sourcetaxonfamilyname:Arthropoda
```

- sample usage 


```
./target/debug/beemorph taxon-id ./interaction/interaction.tsv 4 1131416
./target/debug/beemorph taxon-name ./interaction/interaction.tsv 4 Pompiloides
./target/debug/beemorph taxon-path-name ./interaction/interaction.tsv 4 Insecta
./target/debug/beemorph taxon-path-id ./interaction/interaction.tsv 4 10830
./target/debug/beemorph taxon-genus-id ./interaction/interaction.tsv 4 10830
./target/debug/beemorph taxon-sub-genus-id ./interaction/interaction.tsv 4 10830
./target/debug/beemorph taxon-genus-name ./interaction/interaction.tsv 4 Insecta
./target/debug/beemorph taxon-family-id ./interaction/interaction.tsv 4 235
./target/debug/beemorph taxon-family-name ./interaction/interaction.tsv 4 Arthropoda
```

Gaurav Sablok \
codeprog@icloud.com
