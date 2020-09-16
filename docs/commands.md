# Brewie Commands

CMD  | Name                   | Arguments
---- | ---------------------- | ---------
P80  | Initialize | `toLiter` with one decimal, ???, `mashTemperatureDelta` with 5 decimals, `boilTemperatureDelta` with 5 decimals
P110 | Open water inlet
P111 | Close water inlet
P112 | Open mash inlet
P113 | Close mash inlet
P114 | Open boil inlet
P115 | Close boil inlet
P116 | Open hop 1
P117 | Close hop 1
P118 | Open hop 2
P119 | Close hop 2
P120 | Open hop 3
P121 | Close hop 3
P122 | Open hop 4
P123 | Close hop 4
P124 | Start mash pump
P125 | Stop mash pump
P126 | Start boil pump
P127 | Stop boil pump
P128 | Open cool inlet
P129 | Close cool inlet
P130 | Open cool valve
P131 | Close cool valve
P132 | Open outlet valve
P133 | Close outlet valve
P134 | Open mash return
P135 | Close mash return
P136 | Open boil return
P137 | Close boil return
P150 | Set mash heater target | Target temp, e.g. `320` for 32ºC, `0` to turn off, or `398.2` for 39.82ºC
P151 | Set boil heater target | Target temp, e.g. `320` for 32ºC, `0` to turn off, or `398.2` for 39.82ºC
P205 | Controls some kind of extra logging from the IO board | `0` to turn off, `1` to turn on
P999 | Close all valves

## Startup

The computer sends the P80 message once a second, until a response is received from the IO card. The parameters for this command is read from the file `/usr/share/brewie/config.json`.

The `toLiter` variable is used by the IO card to calculate "Weight in kg" from "Weight Raw value".
