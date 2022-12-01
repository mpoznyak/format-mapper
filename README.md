## Format Mapper
#### Map data from one format to another one providing custom mapping (column -> field, etc.). Supported formats: xls, json, csv (WIP).

Supported mappings (WIP):
- [x] xls to json
- [ ] json to xls
- [ ] csv to json
- [ ] ...

Example:
```shell  
./format-mapper -f ./Book1.xlsx -s Sheet1 -m 0:repository,1:language -i id -g autoincrement -t
```  

This command converts xls data:  
| repository | language |  
| --- | --- |  
| format-mapper | rust |

to json file with the following content:
```json
[{"id":"1","repository" : "repository","language" : "language"},
  {"id":"2","repository" : "format-mapper","language" : "rust"}]
```


```text
Usage: format-mapper [OPTIONS] --file <FILE> --sheet <SHEET> --mapping <MAPPING>

Options:
  -f, --file <FILE>                path to file - required
  -s, --sheet <SHEET>              sheet name - required
  -m, --mapping <MAPPING>          mapping: column_number:field_name - required
  -n, --not-include <NOT_INCLUDE>  number of excel rows which must be not included in parsing
  -t, --trim                       trim whitespaces after parsing - default true
  -i, --id <ID>                    append id field with a specified name
  -g, --igen <ID_GENERATOR>        id generator, required if id flag is specified [possible values: autoincrement, uuid]
  -l, --newline <NEWLINE_REPLACING>  process new line symbols [default: ignore] [possible values: ignore, blank, whitespace]
  -h, --help                       Print help information (use `--help` for more detail)
  -V, --version                    Print version information
```
