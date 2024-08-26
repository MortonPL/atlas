# A script that turns temperature and precipitation data from CRU CL v2.0 dataset
# into a map layer usable with Atlas map generator.

import pandas as pd
import numpy as np
from PIL import Image
from math import floor

CONVERT_TEMP = False
CONVERT_PRECIP = False

DEFAULT_TEMP = 10
DEFAULT_PRECIP = 1000

TARGET_WIDTH = 1000
TAGRET_HEIGHT = 500

WIDTH = 360 * 6
HEIGHT = 180 * 6

def convert_temp(tmp: pd.DataFrame) -> pd.DataFrame:
    new = pd.DataFrame(DEFAULT_TEMP, index=range(HEIGHT), columns=range(WIDTH))
    for _, row in tmp.iterrows():
        # Map real latitude and longitude to one where map origin is in the top left corner.
        lat, long = -(row[0] - 90.0), (row[1] + 180.0)
        y, x = floor((lat / 180) * HEIGHT), floor((long / 360) * WIDTH)
        # Get mean temperature of all months.
        avg = row[2:].mean()
        new.iloc[y, x] = avg
    # Each degree Celsius is 2 temperature units, with 0 C being 100 Tu.
    new = new.applymap(lambda x: min(round(x * 2 + 100), 255))
    return new

def convert_precip(pre: pd.DataFrame) -> pd.DataFrame:
    new = pd.DataFrame(DEFAULT_PRECIP, index=range(HEIGHT), columns=range(WIDTH))
    for _, row in pre.iterrows():
        # Map real latitude and longitude to one where map origin is in the top left corner.
        lat, long = -(row[0] - 90.0), (row[1] + 180.0)
        y, x = floor((lat / 180) * HEIGHT), floor((long / 360) * WIDTH)
        # Get total annual precipitation.
        total = row[2:14].sum()
        new.iloc[y, x] = total
    # Each precipitation unit is 20mm of precipitation.
    new = new.applymap(lambda x: min(round(x / 20), 255))
    return new

def project_and_save(df: pd.DataFrame, name: str):
    # Note: since maps are assumed to use orthographic projection, there's no need to map longitude and latitude.
    img = Image.fromarray(df.to_numpy(dtype=np.uint8), mode="L")
    # Resize to output dimensions using the highest quality resampler.
    img = img.resize((TARGET_WIDTH, TAGRET_HEIGHT), Image.Resampling.BICUBIC)
    img.save(name)

def main():
    # Convert temperature data.
    if CONVERT_TEMP:
        tmp = pd.read_csv("grid_10min_tmp.dat", dtype=float)
        tmp = convert_temp(tmp)
        project_and_save(tmp, "tmp.png")
    else:
        print("Skipping temperature data.")
    # Convert precipitation data.
    if CONVERT_PRECIP:
        pre = pd.read_csv("grid_10min_pre.dat", dtype=float)
        pre = convert_precip(pre)
        project_and_save(pre, "pre.png")
    else:
        print("Skipping precipitation data.")


if __name__ == "__main__":
    main()
