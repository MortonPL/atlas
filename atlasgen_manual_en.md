# Atlas Map Generator Manual

## Window Layout

[Screenshot - Window]()

The application layout consists of two parts: map viewport (to the left) and sidebar (to the right).
The sidebar is divided into three parts: title & menu bar, panel tabs and current panel.

## Menu Bar

### File

#### Export World

Displays a folder dialog. When a directory is chosen, the configuration,
some data layers and climate map are saved as files in that directory.

#### Exit

Exits the application.

### Edit

#### Reset Current Panel

Resets all data in the currently viewed sidebar panel to their default values.

### Config

#### Save Configuration

Displays a file dialog. When a file name is entered or an exisiting file is chosen,
the application configuration data is saved to that path in TOML format.

#### Load Configuration

Displays a file dialog. When a file is chosen, the application configuration is read
from that file if it is in TOML format. Default values will be used if not all
configuration data are present in the TOML file.

#### Reset Configuration

Resets the application configuration data to their default values.

### Layer

#### Load Layer Data

Displays a file dialog. When a file is chosen and it is in the correct image format
(PNG 8-bit RGB color sRGB for the map preview, PNG 8-bit greyscale otherwise) and has matching resolution,
data of the currently viewed map layer will be replaced with that in the image.

#### Save Layer Data

Displays a file dialog. When a file name is entered or an exisiting file is chosen,
data of the currently viewed map layer will be saved to that path as a PNG image.
See previous section for image format details.

#### Clear Layer Data

Clears (batch sets to 0) data of the currently viewed map layer.

#### Render Layer Image

Displays a file dialog. When a file name is entered or an exisiting file is chosen,
the preview of the currently viewed map layer will be saved to that path as an image.
The image format will be PNG 8-bit RGB color sRGB.

### Help

#### About

Displays a window with information about the program.

## Generic Generation Settings

### Noise Algorithm

### Interpolation

### Influence Shape

## Panel Tabs

### General

### Continents

Configuration for continents generation. Each map tile can be either a water tile or a land tile.

The following can be configured:

* Sea level - Height of the global sea level as a fraction (0.0-1.0 range).
Layer data (normalised) below this value will be marked as water, otherwise it will be land,
* Standard noise algorithm with quad point interpolation,
* Standard influence shape.

### Topography

Configuration for topography (height map). Each map tile contains altitude data.
Altitude ranges from 0 (sea level) to 255 (10200 meters), with one altitude unit equal to 40 meters.
Topography is affected by continental data - water tiles are forced to sea level altitude.

The following can be configured:

* Coastal erosion range - Controls how far from the coast (in tiles) coastal erosion affects height.
  The closer to coast, the stronger the reduction in height is. Acceptable value range is from 0 (disabled) to 20.
  Note: long range erosion slows down the generation,
* Standard noise algorithm with quad point interpolation,
* Standard influence shape.

### Temperature

Configuration for temperature map. Each map tile contains temperature data (mean annual at surface level).
Temperature ranges from 0 (-50 degrees Celsius) to 255 (+127,5 degrees), with one temperature unit equal
to 0.5 degrees Celsius and value of 100 equal to 0 degrees.

The following can be configured:

* Moist adiabatic lapse rate (MALR) - Controls how much temperature lowers as altitude rises.
  Expressed in Celsius per kilometer,
* Latitudinal settings (values in degrees Celsisus),
* Standard noise algorithm with quad point interpolation,
* Standard influence shape.

### Precipitation

Configuration for precipitation (rainfall and snowfall) map. Each map tile contains precipitation data (mean annual).
Precipitation ranges from 0 (0 mm) to 255 (5199 mm), with one precipitation unit equal to 20 milimeters of water.

The following can be configures:

* Altitude of maximum precipitation - Controls the *minimum* altitude at which precipitation begins to lower.
  Expressed in meters,
* Precipitation drop - Controls how much precipitation lowers as altitude rises. Expressed in milimeters per meter,
* Latitudinal settings (values in mm),
* Standard noise algorithm with quad point interpolation,
* Standard influence shape.

### Climate

Configuration for climate assigning. Each map tile has assigned an index of a biome from biome list,
based on temperature at precipitation at that location. The exact mapping is controlled by `climatemap.png` file
(PNG 8-bit greyscale, 255x255 pixels), which is essentailly a two dimenstional lookup table:

* Horizontal axis - temperature, from left to right,
* Vertical axis - precipitation, from top to bottom,
* Value at point - index of a biome in the biome list.

Note: the biome list is currently only editable via config file.

Each biome has a name and two color schemes: one for simplified preview (with "similar" biomes sharing colors)
and one for detailed (unique for each biome).

## Tips

* No configuration changes will take effect until you press the "Generate Layer" button for the respective panels.
* Numerical input boxes also act like sliders. Dragging on horizontal axis will decrease or increase value.
* You can drag the edge of the sidebar to adjust its width.
* You can zoom in our out of the map using `+`/`-` keys or mouse wheel.
* If you prefer to work with text files over the GUI, you can save the default configuration and edit its TOML file,
  then load it in Atlas and just generate layers.
