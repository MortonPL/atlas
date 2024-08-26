# Atlas History Simulator Manual

## Window Layout

![Screenshot - Window](atlassim_screenshot.png)

The application layout consists of two parts: map viewport (to the left) and sidebar (to the right).
The sidebar is divided into three parts: title & menu bar, panel tabs and current panel.

## Menu Bar

### File

#### Import Generated World

Displays a folder dialog. When a directory is chosen, the following data
is loaded from that directory from files:

* configuration - `atlassim.toml`,
* preview layer - `preview.png`,
* continental layer - `continents.png`,
* initial topography layer - `topography.png`,
* final topography layer (with sea cutoff and coastal erosion applied) - `realtopography.png`,
* temperature layer - `temperature.png`,
* precipitation layer - `precipitation.png`,
* climate layer - `climate.png`.

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

### Help

#### About

Displays a window with information about the program.

## Panel Tabs (Setup)

Note: names in parentheses (`example`) are sections or keys in the TOML configuration file that refer to the discussed parameters
accessible via the GUI.

### Scenario (`[scenario]`)

Configuration for specific simulation scenario setup, such as amount of starting polities, population, colors.

* \# of Starting Points (`num_starts`) - number of starting points/polities in the simulation,
* Random Start Point Algorithm (`random_point_algorithm`) - algorithm that should be used to automatically set positions (map tiles) of starting points.
  There are 4 random algorithms available:
  * Uniform (`"uniform"`) - tiles have a uniform chance,
  * Weighted (`"weighted"`) - tiles are weighted by their habitability score,
  * WeightedArea (`"weightedarea"`) - tiles are weighted by average hability score of 3x3 area,
  * WeightedSquared(`"weightedsquared"`) -tiles are weighted by habitability squared,
  * WeightedSquaredArea(`"weightedsquaredarea"`) - tiles are weighted by average habitability squared of 3x3 area.
* Starting Land Claim Points (`starting_land_claim_points`) - amount of land claim points initially awarded to each polity,
* Starting Population (`start_pop`) - amount of starting population of each polity,
* Policy Distribution Mean/Deviation (`policy_mean`/`policy_deviation`) - mean/deviation of the normal distribution of policy values,
* Lock All Colors/Positions/Policies (`lock_colors`/`lock_positions`/`lock_policies`) - disable randomization of all polity colors/positions/policies.

Additionally, each starting point/polity (`start_points`) can be manually adjusted:

* Lock Position/Polity Color/Polity Policies (`position_locked`/`color_locked`/`policy_locked`) - disable randomization of this polity's position/color/policies,
* Position (`position`) - map coordiantes of this polity starting position,
* Color (`color`) - RGB888 color for map visualization,
* Population (`population`) - initial population,
* Policies (`policies`) - list of initial policy values.

Available policies:

* Expansionist - region expansion vs region development,
* Competitive - aggresive stance vs peaceful stance,
* Mercantile - wealth production vs industrial production,
* Militarist - military investment vs civilian investment,
* Progressive - research investment vs culture investment,
* Legalist - stability investment.

### Economy (`[rules.economy]`)

Configuration for simulation rules concerning the economy and population:

* Monthly Base Pop Growth (`pop_growth`) - regional population will grow by this multiplier each month (if at full supply and health),
* Maximum Healthcare penalty (`max_health_penalty`) - maximum penalty that can be applied to population growth due to insufficient healthcare,
* Supply/Industry/Wealth Need per Pop (`base_supply_need`/`base_industry_need`/`base_wealth_need`) - amount of supply/industry/wealth one unit of population consumes per month,
* Supply/Industry/Wealth Loss to Chaos (`chaos_supply_loss`/`chaos_industry_loss`/`chaos_wealth_loss`) - amount of supply/industry/wealth one unit of population consumes additionally due to insufficient stability,
* Base Crime Rate (`crime_rate`) - percentage of population that always causes crime (lowers stability),
* Rebelion Speed (`rebelion_speed`) - multiplier to how fast rebelion (low stability due to occupation) increases/decreases,
* Military Industry Storage (`military_stash`) - months worth of military industry output that can be stored for later use,
* Loyalty Storage (`loyalty_stash`) - months worth of loyalty output that can be stored for later use.

For each resource type (in `resources`):

* Supply - consumed by population,
* Industry (in general),
* Civilian industry - used for expansion and development,
* Military industry - used in combat,
* Wealth (in general),
* Research - used for scientific advancement,
* Culture - used for cultural advancement,
* Loyalty - used in combat,
* Industry tributes,
* Wealth tributes,

the following parameters can be adjusted:

* Efficiency (`efficiency`) - how many units of this resource one unit of population can produce monthly,
* Efficiency Over Capacity (`over_cap_efficiency`) - efficiency of resource production when over resource capacity.

### Region (`[rules.region]`)

Configuration for simulation rules concerning the regional development:

* Minimum Size to Split (`min_split_size`) - minimum size in tiles that a region must reach in order to split into two regions,
* New City Cost (`new_city_cost`) - cost of establishing a new city/region,
* Land Claim Cost (`land_claim_cost`) - cost of claiming a new map tile,
* Expansion Cost Increase Per Region (`sprawl_penalty`) - flat increase to land claim and new city cost for each polity region,
* Base Expansion Speed (`base_exp_speed`) - conversion modifier from civilian industry to land claim points and new city points,
* Base Development Speed (`base_dev_speed`) - conversion modifier from civilian industry to development points and strucutre points,
* Development Level Cost (`dev_level_cost`) - percent increase in development cost per existing development level,
* Maximum Development Level (`max_dev_level`) - maximum development & structure level,
* Development Bonus (`dev_bonus`) - bonus to supply/industry/wealth production per development level,
* Base Capacity (`base_capacity`) - capacity provided per structure level.

For each structure (in `structures`):

* Hospital - provides regional health power,
* Manufacture - provides civilian industry capacity,
* Forge - provides military industry capacity,
* University - provides research capacity,
* Amphitheater - provides culture capacity,
* Couthouse - provides reegional security power,
* Fortress - provides fortification level,

the following parameters can be adjusted:

* Strength (`strength`) - multiplier to base capacity,
* Cost (`cost`) - multiplier to development speed.

### Diplomacy (`[rules.diplomacy]`)

Configuration for simulation rules concerning inter-polity diplomacy and policies:

* Initial Peace (`initial_peace_length`) - length in months of the forced peace period at the start of the simulation,
* Truce Length (`truce_length`) - length in months of a truce period after a conflict ends,
* Policy Change Time Mean/Deviation (`policy_time_mean`/`policy_time_dev`) - mean/deviation of the normal distribution of time for policy change (in years),
* Relations Change Speed (`relations_speed`) - conversion modifier from competitive policy value to mutual relations value,
* Base Relations Improvement (`base_good_shift`) - bias towards positive mutual relations,
* Ally/Friend threshold (`ally_threshold`/`friend_threshold`) - minimum relations threshold to reach to be considered allies/friends,
* Rival/Enemy threshold (`rival_threshold`/`enemy_threshold`) - maximum relations threshold to reach to be considered rivals/enemies,
* Region Claim Difficulty (`claim_difficulty`) - multiplier to loser's conflict contribution score when making regional claims,
* Tribute Length (`tribute_time`) - length of tribute payments in months,
* Economy to Tribute (`tribute_ratio`) - percent of resources per lost conflict.

### Science (`[rules.science]`)

Configuration for simulation rules concerning scientific research:

* Major/Minor Level Speed (`speed_major`/`speed_minor`) - conversion multiplier from science resource to progress in major/minor field level,
* Maximum Major/Minor Level (`max_level_major`/`max_level_minor`) - maximum possible major/minor field level,
* Major/Minor Level Bonus (`bonus_major`/`bonus_minor`) - multiplier to bonuses provided by a field per major/minor level,
* Base Decay (`base_decay`) - base progress decay for each field,
* Major Level Decay (`level_decay`) - additional progress decay per field major level,
* Major Level Difficulty Increase (`level_difficulty`) - percent of progress cost increased per major level,

For each scientific field (in `fields`):

* Geoscience - increased supply/industry/wealth production,
* Medicine - increased healthcare,
* Engineering - increased civilian industry,
* Metallurgy - increased military industry,
* Philosophy - increased culture,
* Mathematics - increased research,
* Management - increased loyalty,
* Law - increased stability,
* Linguistics - increased inter-polity relation strength,
* Military Tech - increased combat strength,

the following parameters can be adjusted:

* Strength (`strength`) - multiplier to bonuses provided by this field,
* Speed (`speed`) - multiplier to research speed.

### Culture (`[rules.culture]`)

Configuration for simulation rules concerning cultural activities:

* Base Speed (`base_speed`) - conversion multiplier from culture resource to progress in a tradition,
* Base Decay (`base_decay`) - base progress decay for each tradition,
* Maximum Level (`max_level`) - maximum possible tradition level,
* Level Bonus (`level_bonus`) - multiplier to bonuses provided by a tradition per level,
* Level Decay (`level_decay`) - additional progress decay per tradition level,
* Overflow Culture to Heritage Ratio (`heritage_ratio`) - ratio of tradition porgress overflow converted to heritage points,
* Great Event Heritage Cost (`great_event_heritage`) - accumulated heritage is divided by this to get great event probability,
* Great Event Max Chance (`great_event_chance_max`) - maximum possible great event probability regardless of accumulated heritage,
* Great Person Chance (`great_person_chance`) - probability that a great event will be a great person,
* Great Work Bonus (`great_work_bonus`) - tradition levels added per great work,
* Great Person Bonus (`great_person_bonus`) - tradition levels added per active great person,
* Great Person Duration (`great_person_duration`) - duration of great person activity in months,

For each tradition type (in `traditions`):

* Pioneering - used in expansion,
* Monumentality - used in development,
* Curiosity - used in scientific advancement,
* Creativity - used in cultural advancement,
* Prosperity - used in production,
* Authority - used for stability,
* Diplomacy - used in inter-polity relations,
* Supremacy - used in combat,

the following parameters can be adjusted:

* Strength (`strength`) - multiplier to bonuses provided by this field,
* Speed (`speed`) - multiplier to research speed.

### Combat (`[rules.combat]`)

Configuration for simulation rules concerning conflicts:

* Action Weights (Attacker/Deffender) (`action_weights_attacker`/`action_weights_defender`) - weights of combat actions (assault/maneouver/charge/rally/siege/fortify) for attacker/defender,
* Assault Bonus (`assault_bonus`) - multiplier to material attack and defence when assaulting,
* Maneouver Bonus (`maneouver_bonus`) - multiplier to morale attack and defence when maneouvring,
* Charge Bonus (`charge_bonus`) - multiplier to any attack and penalty to any defence when charging,
* Rally Bonus (`rally_bonus`) - multiplier to any defence and penalty to any attack when rallying,
* Siege Bonus (`siege_bonus`) - conversion factor of any attack into siege damage when laying siege,
* Siege Penalty (`siege_penalty`) - penalty to any attack and defence when laying siege,
* Fortify Bonus (`fortify_bonus`) - multiplier to fortification level when fortifying,
* Fortify Penalty (`fortify_penalty`) - penalty to any attack when fortifying,
* Military Size Ratio (`military_size`) - military size can be up to this fraction of total population, per active conflict,
* Base Mobilization Speed (`base_mobilization`) - fraction of recruitable population that is mobilized into military per month,
* Mobilization From Militarist Policy (`militarist_mobilization`) - bonus to mobilization speed based on militarist policy,
* Initial Mobilization Time (`mobilization_build_up`) - months to spend mobilizing troops before committing to a conflict,
* Combat Randomness (`randomness`) - random strength multiplier will be in [1.0 - this; 1.0 + this] range,
* Material/Morale Damage Fatality/Fragility (`fatality`/`fragility`) - conversion factor from total material/morale strength to material/morale damage,
* Material/Morale Advantage Power (`material_advantage`/`morale_advantage`) - exponent of material/morale advantage ratio,
* Morale Breakdown Multiplier (`breakdown`) - conversion factor from excess morale damage to additional material damage when suffering a breakdown,
* Morale to Material Ratio Cap (`morale_cap`) - accumulated morale can this times larger than material,
* Equipment to Manpower Ratio (`equipment_manpower_ratio`) - each unit of material (manpower) requires this many units of military industry,
* Damage to Fort Ratio (`fort_damage`) - conversion factor from excess material damage to fort damage,
* Monthly Attacker/Defender Attrition (`base_attacker_attrition`/`base_defender_attrition`) - base monthly attrition increase for attacker/defender,
* Attrition From Combat Damage (`combat_attrition`) - attrition multiplier to quotient of material loss to total population,
* Attrition From Civilian Damage (`civilian_attrition`) - attrition multiplier to quotient of unabsorbed damage to total population,
* Civilian Damage From Combat Modifier (`civilian_damage`) - civilian damage multiplier to quotient of unabsorbed damage to total population,
* Maximum Civilian Damage Ratio (`civilian_damage_max`) - maximum percent of civilian damage dealt per month,
* Base Rebel Rate in Claimed Regions (`base_rebel_rate`) - base rebel rate in claimed regions,
* Rebel Damage to Infrastructure (`rebel_structure_damage`) - multiplier to one-time development damage in rebelious regions,

### Climate (`[climate]`)

Climate settings carried over from Atlas Map Generator. Each map tile has assigned an index of a biome from biome list.

There are two preview modes for this layer:

* Simplified color,
* Detailed color,

Each biome (in `biomes`) has a name (`name`) and the following properties:

* Color (`color`) - Color to use for this climate in the detailed climate preview mode. Each biome should have a unique color,
* Color (simplified view) (`simple_color`) - Color to use in the simplified climate preview mode. Similar biomes should share colors,
* Resources (`deposits`) - List of resource deposit IDs that this biome provides with given probability.
  Note: this field can be empty, as all resources have already been assigned in Atlas Map Generator or manually.
* Habitability (`habitability`) - Weight used for assinging starting locations and border expansion costs.

Note: adding or removing biomes from the list is possible only via config file.

## Panel Tabs (Running)

### Selected

Information about the currently selected region:

* regional civilian population,
* total public security power & public health power,
* regional stability & healthcare (as % of population covered),
* number of map tiles occupied by region,
* accumulated land claim points & new city points,
* development level of the region,
* list of regional key structures and their levels,
* list of resource deposits available in this region.

### Polity

General information about the currently selected polity:

* the internal polity ID and map color,
* total number of regions,
* total civilian population,
* average stability & healthcare of all regions (as % of population covered),
* date of the next change of policies,
* list of current policies,
* list of neighbours: their polity ID and mutual relation value (above 0 is positive, below 0 is negative),
* list of pending tributes: number of monthly payments left, receiving polity ID and percent of tribute fund to pay per payment.

### Economy

Information about economy output and employment structure of the currently selected polity:

* accumulated yearly civilian industry,
* output of each resource this month,
* amount of industry and wealth tribute paid this month,
* total polity population (civilian + military),
* population count employed per sector: military, supply, industry, wealth.

### Science

Information about scientific progress of the currently selected polity:

* accumulated yearly research points,
* list of scientific fields with their major (left value) and minor (right value) levels.

### Culture

Information about cultural progress of the currently selected polity:

* accumulated yearly culture points,
* list of traditions with their primary level (left value) and great event level (right value),
* list of accumulated heritage for traditions (measured in culture points),
* list of created great works and great people containing date of creation and associated tradition.

### Combat

List of all conflicts that the currently selected polity is a part of and detailed information about them:

* conflict start date,
* primary attacker polity ID,
* primary defender polity ID,
* list of all conflict members, divided into attackers and defenders:
  * polity ID and map color,
  * months left to mobilize troops before entering combat,
  * material and morale strength committed to the conflict,
  * current attrition rate,
  * total contribution to conflict,
  * strength of all polity fortifications left,
  * chosen combat action for this month and engagement status.

## Tips

* Numerical input boxes also act like sliders. Dragging on horizontal axis will decrease or increase value.
* You can drag the edge of the sidebar to adjust its width.
* You can zoom in or out of the map using the mouse wheel.
* You can drag the map around while the right mouse button is pressed.
* If you prefer to work with text files over the GUI, you can save the default configuration and edit its TOML file,
  then load it in Atlas History Simulator and only generate layers.
