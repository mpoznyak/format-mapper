## Format Mapper
### Provide some mapping and let the converter do the rest

Supported mappings (WIP):
- [x] xls -> json
- [ ] json -> xls
- [ ] csv -> json
- [ ] ...

Example:
```shell  
./format-mapper -f ./Book1.xlsx -s Sheet1 -m 1:repository,2:language -i id -g autoincrement -n 1 -t
```  

This command converts Sheet1 of xls file with data  
| repository | language |  
| --- | --- |  
| format-mapper | rust |


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
  -h, --help                       Print help information (use `--help` for more detail)
  -V, --version                    Print version information
```