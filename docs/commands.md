# Brewie Commands

CMD  | Name                   | Arguments
---- | ---------------------- | ---------
P80  | Initialize | `toLiter` with one decimal, ???, `mashTemperatureDelta` with 5 decimals, `boilTemperatureDelta` with 5 decimals
P103 | Enqueue a step | See below
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
P205 | Controls the fans, and some kind of extra logging from the IO board | `0` to turn off, `1` to turn on
P999 | Close all valves

## Startup

The computer sends the P80 message once a second, until a response is received from the IO card. The parameters for this command is read from the file `/usr/share/brewie/config.json`.

The `toLiter` variable is used by the IO card to calculate "Weight in kg" from "Weight Raw value".

## Step

The `P103` command is used to enqueue a step. It seems like the computer first sends two steps, and then one more as each step is completed. This way the IO board always knowns what to do immediately after the step it is currently working on.

Offset | Type | Argument
------ | ---- | ----
0      | Int  | Step number, starting at `0`
1      | Bool | Water inlet (`0` = close, `1` = open)
2      | Bool | Mash inlet valve (`0` = close, `1` = open)
3      | Bool | Boil inlet valve (`0` = close, `1` = open)
4      | Int  | Mash tank target temp (`670` for 67ºC, `0` to turn off)
5      | Int  | Boil tank target temp (`670` for 67ºC, `0` to turn off)
6      | Bool | Hop Cage 1 (`0` = close, `1` = open)
7      | Bool | Hop Cage 2 (`0` = close, `1` = open)
8      | Bool | Hop Cage 3 (`0` = close, `1` = open)
9      | Bool | Hop Cage 4 (`0` = close, `1` = open)
10     | Bool | Cool valve (or inlet?) (`0` = close, `1` = open)
11     | Int  | Cool inlet (or valve?) (`0` = close, `255` = open)
12     | Bool | ??? (could be the eleventh valve, thus always `0`)
13     | Int  | Mash tank pump (`0` = off, `255` = on)
14     | Int  | Boil tank pump (`0` = off, `255` = on)
15     | Int  | ??? (potentially water intake)
16     | Int  | Step time in seconds
17     | Int  | ??? (maybe step completion type? `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, `10`)
18     | Int  | ??? (`0`, `2`, `3`) (`2` seems to be "sparge", `3` seems to be "boil")
19     | Bool | Mash return valve (`0` = close, `1` = open)
20     | Bool | Boil return valve (`0` = close, `1` = open)
