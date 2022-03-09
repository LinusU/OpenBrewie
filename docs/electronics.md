# Electronics

## Power board

### Input

Board name | Connection | Wire marking
---------- | ---------- | ------------
`PE`       | Spade type lug ¼ inch | Green/Yellow
`N`        | Spade type lug ¼ inch | Brown
`L`        | Spade type lug ¼ inch | Blue

### Output

Board name | Connection | Wire marking
---------- | ---------- | ------------
`PE`       | Spade type lug ¼ inch | *not connected*
`SUPPLY_N` | Spade type lug ¼ inch | `NO17`
`HEATER2_N` | Spade type lug ¼ inch | `NO21`
`HEATER1_N` | Spade type lug ¼ inch | `NO20`
`SUPPLY_L` | Spade type lug ¼ inch | `NO30` to switch, then `NO18` to supply
`HEATER2_L` | Spade type lug ¼ inch | `NO16`
`HEATER1_L` | Spade type lug ¼ inch | `NO15`

## Power Supply

Mean Well LRS-35-12

### Connections

Board name | Target | Connection | Wire marking
---------- | ------ | ---------- | ------------
`L`        | `SUPPLY_N` | Fork type lug | `NO17`
`N`        | to power switch, then `SUPPLY_L` | Fork type lug | `NO18` to switch, then `NO30`
⏚          | *not connected* | Fork type lug
`-V` and `+V` | `PW` on Arduino board | Fork type lug | `NO19`

## Arduino board

### Valves

Board name | Valve | Wire marking
---------- | ----- | ------------
`VALVE1`   | Mash Inlet | Hand drawn `1`
`VALVE2`   | Mash Return | Hand drawn `2`
`VALVE3`   | Boil Inlet | Hand drawn `3`
`VALVE4`   | Boil Return | Hand drawn `4`
`VALVE5`   | Hop 3 | Hand drawn `5`
`VALVE6`   | Hop 4 | Hand drawn `6`
`VALVE7`   | Hop 1 | Hand drawn `7`
`VALVE8`   | Hop 2 | Hand drawn `8`
`VALVE9`   | *not connected*
`VALVE10`  | Cool | Hand drawn `10`
`VALVE11`  | Outlet | Hand drawn `11`
`INLETVALVE1` | Cool Inlet  | `NO7`
`INLETVALVE2` | Water Inlet | `NO14`

### Pumps

Board name | Pump | Wire marking
---------- | ---- | ------------
`PUMP1`    | Mash | `NO13`
`PUMP2`    | Boil | `NO6`

### Temperature Sensors

Board name | Location | Wire marking
---------- | -------- | ------------
`HALL1`    | Outside of electronics box | `NO29`
`HALL2`    | *not connected*
`TEMP1`    | Against boil tank | `NO10`
`TEMP2`    | Against mash tank | `NO9`

### Pressure Sensors

Board name | Wire marking
---------- | ------------
`L` ???? `OR` | `NO5` (four green cables)

### Serial Ports

Board Name | Wire marking
---------- | ------------
`SERIAL0`  | `NO4` (five red cables)
`SERIAL1`  | *not connected*

### Heat

Board Name | Heat | Wire marking
---------- | ---- | ------------
`HEAT1`    | Mash tank | `NO11`
`HEAT2`    | Boil tank | `NO12`

### Power Input

Board Name | Voltage | Wire marking
---------- | ------- | ------------
`PW`       | `12V`   | `NO19`
