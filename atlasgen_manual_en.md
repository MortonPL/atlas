# Atlas Map Generator Manual

## Window Layout

![Screenshot - Window](atlasgen_screenshot.png)

The application layout consists of two parts: map viewport (to the left) and sidebar (to the right).
The sidebar is divided into three parts: title & menu bar, panel tabs and current panel.

## Menu Bar

### File

#### Import World

Displays a folder dialog. When a directory is chosen, the following data
is loaded from that directory from files:

* configuration - `atlasgen.toml`,
* preview layer - `preview.png`,
* continental layer - `continents.png`,
* initial topography layer - `topography.png`,
* final topography layer (with sea cutoff and coastal erosion applied) - `realtopography.png`,
* temperature layer - `temperature.png`,
* precipitation layer - `precipitation.png`,
* climate layer - `climate.png`,
* climate map - `climatemap.png`.

#### Export World

Displays a folder dialog. When a directory is chosen, world data is saved as files
in that directory. See previous action "Import World" for a list of all relevant files.

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
(PNG 8-bit RGBA color sRGB for the map preview, PNG 8-bit greyscale otherwise) and has matching resolution,
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
The image format will be PNG 8-bit RGBA color sRGB.

### Help

#### About

Displays a window with information about the program.

## Generic Generation Settings

Note: names in parentheses (`example`) are sections or keys in the TOML configuration file that refer to the discussed parameters
accessible via the GUI.

### Noise Algorithm (`algorithm`)

This program uses two dimensional fractal noise as the basis of the generation process, with sample data
scaled to [0.0; 1.0] range.
There are 4 base noise algorithms available:

* Perlin (`perlin`)
* PerlinSurflet (`perlinsurflet`)
* OpenSimplex (`opensimplex`)
* SuperSimplex (`supersimplex`)
* FromImage (`fromimage`) - a special case. No data is generated, useful for when users supply exisitng data and don't want it to be overriden.

The following parameters can be used to customize generation output:

* Seed (`seed`) - the seed of random numbers. Changing the seed gives completely different results,
* Detail (number of octaves) (`detail`) - number of layers of noise overlaid on top of each other. Affects the overall level of detail.
  High level of zoom (low scale) requires higher detail for good results. High detail makes generation slower,
* Scale (frequency) (`frequency`) - frequency of sampling. High scale gives more zoomed out results,
* Neatness (lacunarity) (`neatness`) - multiplier to frequency for consecutive layers of noise. Small changes give different results,
  large increase will make shapes less defined,
* Roughness (persistance) (`roughness`) - power of amplitude for consecutive layers of noise. Low values give blurry shapes, high values give
  rough, high contrast shapes but also increase value,
* Bias (`bias`) - offset to the output *value* in [-1.0; 1.0] range,
* Offset (`offset`) - horizontal and vertical offset of the output. Offset should be scaled when frequency and lacunarity is changed.

### Interpolation (`midpoint`)

Output (in [0.0; 1.0] range) can be further modifed using three segment (four control point) linear interpolation.
The start and end points have fixed position of 0.1 and 1.0 respectively, while two middle points
(`midpoint_position` and `midpoint2_position`) can be freely moved on the X axis.
Value (Y axis) can be customized for all 4 points (`start`, `midpoint`, `midpoint2` and `end`).
This allows great flexibility in manipulating output ranges and distributions, i.e.:

* Scaling values,
* Reversing values,
* Creating steep slopes,
* Approximating nonlinear transformations.

### Latitudinal Interpolation (`latitudinal`)

Temperature and precipitation use noise algorithms only as supplementary (optional) source of data. The main source of data for these layers
is latitude (map position on Y axis) based linear interpolation, with 9 fixed control points loosely based on circles of lattitude:

* Equator (0 degrees) (`equator_value`),
* Tropics (23 degrees) (`north_tropic_value` and `south_tropic_value`),
* Temperate zones (46 degrees) (`north_temperate_value` and `south_temperate_value`),
* Arctic/Antarctic (69 degrees) (`north_arctic_value` and `south_arctic_value`),
* Poles (90 degrees) (`north_pole_value` and `south_pole_value`),

An additional custom seeting is available, "Non-Linear Tropic Bias" (`non_linear_tropics`). When enabled, the interpolation between tropics and temperate zones
becomes non-linear (in favor of tropics) which might produce better results when creating dry tropical deserts.

### Influence Shape (`influence_shape`)

Each layer can be optionally affected by a special layer called the "influence" map. It is a separate layer
which can scale map data up or down, with intensity controlled by "Influence Strength" setting (`influence_strength`),
depending on chosen mode (`influence_mode`):

* Scale down (`scaledown`) - influence values below 1.0 will scale data at that point down.
  This can be used to erase features outside the point of interest, i.e. to remove land at map corners,
* Scale up (`scaleup`) - influence values above 0.0 will scale data up. This can be used to emphasize features of specific locations,
* Scale down / up (`scaleupdown`) - influence values above 0.5 will scale data up, while below 0.5 will scale data down. This is a combination
  of the previous two modes.

Influence map can be generated in many ways, with most of them supporting interpolation as well (`midpoint`):

* None (`none`) - influence map is not applied,
* Circle (`circle`) - creates a circle defined by position (`offset`, offset from map center) and radius (`radius`). Points inside the circle
  have assigned value equal distance from circle center divided by its radius,
* Strip (`strip`) - creates a segment with a circle at each end. The following parameters are available: Segment center (`offset`, offset from map center),
  segment length (`length`), angle between the segment and horizontal axis (`angle`), segment thickness (`thickness`, also acts as circle radius),
  option to flip the strip horizontally (`flip`),
* Fbm (`fbm`) - standard noise algorithm (`algorithm`),
* FromImage (`fromimage`) - influence map is preserved as is, i.e. when loaded from file.

## Panel Tabs

Note: names in parentheses (`example`) are sections or keys in the TOML configuration file that refer to the discussed parameters
accessible via the GUI.

### General (`[general]`)

Configuration for the map in general as well as preview.

The following can be configured:

* Altitude limit for preview (`altitude_limit`) - Controls the altitude maximum for preview altitude shading. If above 0, tiles will become darker as they come closer to the maximum,
* Preview height levels (`height_levels`) - Controls how many discrete shading levels should be shown in the preview,
* Preview color display (`color_display`) - Controls how the tiles are colored when generating previews:
  * Topography (`topography`) - altitude based color palette (green-yellow-brown-grey),
  * Simplified climate (`simplifiedclimate`) - climate (biome) based color palette, using simplified biome colors,
  * Detailed climate (`detailedclimate`) - climate (biome) based color palette.
* Preview world model (`preview_model`) - Controls if the world map should be previewed as a flat map or as a globe,
* World size (`world_size`) - Horizontal (longitudinal) and vertical (latitudinal) size of the world, in tiles.

### Continents (`[continents]`)

Configuration for continents generation. Each map tile can be either a water tile or a land tile.

The following can be configured:

* Sea level (`sea_level`) - Height of the global sea level as a fraction (0.0-1.0 range).
Layer data (normalised) below this value will be marked as water, otherwise it will be land,
* Standard noise algorithm with quad point interpolation (`algorithm`),
* Standard influence shape (`influence_shape`).

### Topography (`[topography]`)

Configuration for topography (height map). Each map tile contains altitude data.
Altitude ranges from 0 (sea level) to 255 (10200 meters), with one altitude unit equal to 40 meters.
Topography is affected by continental data - water tiles are forced to sea level altitude.

The following can be configured:

* Coastal erosion range (`coastal_erosion`) - Controls how far from the coast (in tiles) coastal erosion affects height.
  The closer to coast, the stronger the reduction in height is. Acceptable value range is from 0 (disabled) to 20.
  Note: long range erosion slows down the generation,
* Standard noise algorithm with quad point interpolation (`algorithm`),
* Standard influence shape (`influence_shape`).

### Temperature (`[temperature]`)

Configuration for temperature map. Each map tile contains temperature data (mean annual at surface level).
Temperature ranges from 0 (-50 degrees Celsius) to 255 (+77,5 degrees), with one temperature unit equal
to 0.5 degrees Celsius and value of 100 equal to 0 degrees.

The following can be configured:

* Moist adiabatic lapse rate (MALR) (`lapse_rate`) - Controls how much temperature lowers as altitude rises.
  Expressed in Celsius per kilometer,
* Latitudinal settings (values in degrees Celsisus) (`latitudinal`),
* Noise strength - Scales down noise algorithm output (`algorithm_strength`),
* Standard noise algorithm with quad point interpolation (`algorithm`),
* Standard influence shape (`influence_shape`).

### Precipitation (`[precipitation]`)

Configuration for precipitation (rainfall and snowfall) map. Each map tile contains precipitation data (mean annual).
Precipitation ranges from 0 (0 mm) to 255 (5100 mm), with one precipitation unit equal to 20 milimeters of water.

The following can be configures:

* Altitude of maximum precipitation (`amp_point`) - Controls the *minimum* altitude at which precipitation begins to lower.
  Expressed in meters,
* Precipitation drop (`drop_per_height`) - Controls how much precipitation lowers as altitude rises. Expressed in milimeters per meter,
* Latitudinal settings (values in mm) (`latitudinal`),
* Noise strength (`algorithm_strength`) - Scales down noise algorithm output,
* Standard noise algorithm with quad point interpolation (`algorithm`),
* Standard influence shape (`influence_shape`).

### Climate (`[climate]`)

Configuration for climate assigning. Each map tile has assigned an index of a biome from biome list,
based on temperature at precipitation at that location. The exact mapping is controlled by `climatemap.png` file
(PNG 8-bit greyscale, 255x255 pixels), which is essentailly a two dimenstional lookup table:

* Horizontal axis - temperature, from left to right,
* Vertical axis - precipitation, from top to bottom,
* Value at point - index of a biome in the biome list.

There are two preview modes for this layer:

* Simplified color,
* Detailed color,

Each biome (`biomes`) has a name (`name`) and the following properties:

* Color (`color`) - Color to use for this climate in the detailed climate preview mode. Each biome should have a unique color,
* Color (simplified view) (`simple_color`) - Color to use in the simplified climate preview mode. Similar biomes should share colors,
* Resources (`deposits`) - List of resource deposit IDs that this biome provides with given probability.
* Habitability (`habitability`) - Weight used in the Atlas History Simulator for assinging starting locations and border expansion costs.

Note: adding or removing biomes from the list is possible only via config file.

### Deposits (`[deposits]`)

Configuration for resource deposits. World resources are grouped into square chunks of specified size (`chunk_size`),
with each tile contributing resource deposits to its chunk. Deposits are not visualized in any way and
have no impact on layers of world generation, but are essential to the Atlas History Simulator.

Each resource deposit type (`types`) has a name (`name`) and the following properties:

* Random Deposit Chance (`gen_chance`) - Probability of appearing in a tile regardless of biome.
* Deposit Average Size (`gen_average`) - Mean of the deposit size normal distribution. Larger deposits provide more resources.
* Deposit Size Deviation (`gen_deviation`) - Deviation of the deposit size normal distribution.
* Supply Points (`supply`) - Amount of supply resources provided per deposit size.
* Industry Points (`industry`) - Amount of industry resources provided per deposit size.
* Wealth Points (`wealth`) - Amount of wealth resources provided per deposit size.

Each deposit chunk (`chunks`) has the following properties:

* Land Tile Count (`tile_count`) - Number of continental tiles within the chunk.
* Deposits (`deposits`) - List of deposit types contained and their total size in the chunk.

## Tips

* No configuration changes will take effect until you press the "Generate Layer" button for the respective panels.
* Numerical input boxes also act like sliders. Dragging on horizontal axis will decrease or increase value.
* You can drag the edge of the sidebar to adjust its width.
* You can zoom in or out of the map using the mouse wheel.
* You can drag the plane in flat preview mode and rotate the globe in globe preview mode while pressing the right mouse button,
* If you prefer to work with text files over the GUI, you can save the default configuration and edit its TOML file,
  then load it in Atlas Map Generator and only generate layers.
