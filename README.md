# about

Accessibility utility to move mouse cursor using keyboard with
[binary search algorithm](https://en.wikipedia.org/wiki/Binary_search_algorithm)


# requirements

the rust development enviroment
```bash
curl https://sh.rustup.rs -sSf | sh
```

a tile window manager running over x11 like i3 or leftwm

the mickey executable
```bash
cargo install mickey
```

# usage

```bash
# Move the cursor to center of screen and restore the cursor speed in all axis
mickey centralize

# Emulate click with mouse at current cursor position and restore the cursor speed in all axis
mickey click left

# Move cursor to left or right and decrease in half the cursor speed at axis X
mickey move left
mickey move right

# Move cursor to top or bottom and decrease in half the cursor speed at axis Y
mickey move top
mickey move bottom

# Emulate a hold or release of left mouse button
mickey hold

# Show help with another options
mickey
````

# Example
Example of leftwm config

```toml
#~/.config/leftwm/config.toml
# ...
[[keybind]]
modifier = ["Alt"]
key = "1"
command = "Execute"
value = "mickey click -r 18 scroll-up"

[[keybind]]
modifier = ["Alt"]
key = "2"
command = "Execute"
value = "mickey click -r 6 scroll-up"

[[keybind]]
modifier = ["Alt"]
key = "3"
command = "Execute"
value = "mickey click -r 6 scroll-down"

[[keybind]]
modifier = ["Alt"]
key = "4"
command = "Execute"
value = "mickey click -r 6 scroll-down"

[[keybind]]
modifier = ["Alt"]
key = "q"
command = "Execute"
value = "mickey click -r 3 left"

[[keybind]]
modifier = ["Alt"]
key = "w"
command = "Execute"
value = "mickey move top"

[[keybind]]
modifier = ["Alt"]
key = "s"
command = "Execute"
value = "mickey move bottom"

[[keybind]]
modifier = ["Alt"]
key = "a"
command = "Execute"
value = "mickey move left"

[[keybind]]
modifier = ["Alt"]
key = "d"
command = "Execute"
value = "mickey move right"


[[keybind]]
modifier = ["Alt"]
key = "c"
command = "Execute"
value = "mickey click left"


[[keybind]]
modifier = ["Alt"]
key = "f"
command = "Execute"
value = "mickey hold"


[[keybind]]
key = "e"
modifier = ["Alt"]
command = "Execute"
value = "mickey centralize"
```