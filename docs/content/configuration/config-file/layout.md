# Layout

!!! Warning

    This section is in progress, and is just copied from the old documentation.

bottom supports customizable layouts via the config file. Currently, layouts are controlled by using TOML objects and arrays.

For example, given the sample layout:

```toml
[[row]]
  [[row.child]]
  type="cpu"
[[row]]
    ratio=2
    [[row.child]]
      ratio=4
      type="mem"
    [[row.child]]
      ratio=3
      [[row.child.child]]
        type="temp"
      [[row.child.child]]
        type="disk"
```

This would give a layout that has two rows, with a 1:2 ratio. The first row has only the CPU widget.
The second row is split into two columns with a 4:3 ratio. The first column contains the memory widget.
The second column is split into two rows with a 1:1 ratio. The first is the temperature widget, the second is the disk widget.

This is what the layout would look like when run:

![Sample layout](../../assets/screenshots/config/layout/sample_layout.webp)

Each `[[row]]` represents a _row_ in the layout. A row can have any number of `child` values. Each `[[row.child]]`
represents either a _column or a widget_. A column can have any number of `child` values as well. Each `[[row.child.child]]`
represents a _widget_. A widget is represented by having a `type` field set to a string.

The following `type` values are supported:

|                                     |                          |
| ----------------------------------- | ------------------------ |
| `"cpu"`                             | CPU chart and legend     |
| `"mem", "memory"`                   | Memory chart             |
| `"net", "network"`                  | Network chart and legend |
| `"proc", "process", "processes"`    | Process table and search |
| `"temp", "temperature"`             | Temperature table        |
| `"temp_graph", "temperature_graph"` | Temperature graph        |
| `"disk"`                            | Disk table               |
| `"disk_io_graph"`                   | Disk I/O graph           |
| `"power"`                           | Raspberry Pi PMIC power  |
| `"empty"`                           | An empty space           |
| `"batt", "battery"`                 | Battery statistics       |

Each component of the layout accepts a `ratio` value. If this is not set, it defaults to 1.

Furthermore, you can have duplicate widgets.

For an example, look at the [default config](https://github.com/ClementTsang/bottom/blob/main/sample_configs/default_config.toml), which contains the default layout.

## Hiding widgets

Set `hidden = true` on any `[[row.child]]` or `[[row.child.child]]` widget
entry, then restart bottom. The widget is removed from the layout and keyboard
navigation; remaining widgets fill the space. Empty rows and columns are removed
as well. Remove the setting or use `hidden = false` to show it again. Keep at
least one widget visible. These settings apply to custom layouts, not basic mode.

```toml
[[row]]
  [[row.child]]
  type = "cpu"
  [[row.child]]
  type = "disk"
  hidden = true
```

## Raspberry Pi 5 power graph

Add a widget with `type = "power"` to graph PMIC rail power in watts. The graph
supports the usual zoom, expand, and freeze controls. Collection runs only when
a power widget is present in the visible layout.

```toml
[[row]]
  [[row.child]]
  type = "power"
```

Requires Linux on a Raspberry Pi 5 with `vcgencmd pmic_read_adc` working for the
user running bottom. If it requires root, run `sudo -v` in the same terminal
before starting bottom. The collector tries `sudo -n vcgencmd pmic_read_adc`
when direct access fails; it never prompts for or stores your password. Renew
the authorization with `sudo -v` if it expires. Missing or invalid readings
display as unavailable and leave
a gap in the graph. Power is the sum of each reported rail's voltage × current.
This is **PMIC output power**, not total USB-C input or wall power: USB devices,
other direct 5V loads, and conversion losses are not included. See the
[Raspberry Pi hardware documentation](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html).

A ready-to-use layout is included in `sample_configs/rpi5.toml`:

```sh
cargo run --release -- -C sample_configs/rpi5.toml
```
