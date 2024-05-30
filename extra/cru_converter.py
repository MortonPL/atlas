# A script that turns temperature and precipitation data from CRU CL v2.0 dataset
# into a map layer usable with Atlas map generator.

import pandas as pd
import numpy as np
from PIL import Image
from math import floor

DO_TEMP = False
DO_PRE = False

TARGET_WIDTH = 1000
TAGRET_HEIGHT = 500

WIDTH = 360 * 6
HEIGHT = 180 * 6

def convert_tmp(tmp: pd.DataFrame) -> pd.DataFrame:
    new = pd.DataFrame(0, index=range(HEIGHT), columns=range(WIDTH))
    count = pd.DataFrame(0, index=range(HEIGHT), columns=range(WIDTH))
    for _, row in tmp.iterrows():
        lat, long = -(row[0] - 90.0), (row[1] + 180.0)
        y, x = floor((lat / 180) * HEIGHT), floor((long / 360) * WIDTH)
        avg = row[2:].mean()
        count.iloc[y, x] += 1
        new.iloc[y, x] += (avg - new.iloc[y, x]) / count.iloc[y, x]
    new = new.applymap(lambda x: round(x * 2 + 100))
    return new

def convert_pre(pre: pd.DataFrame) -> pd.DataFrame:
    new = pd.DataFrame(0, index=range(HEIGHT), columns=range(WIDTH))
    for _, row in pre.iterrows():
        lat, long = -(row[0] - 90.0), (row[1] + 180.0)
        y, x = floor((lat / 180) * HEIGHT), floor((long / 360) * WIDTH)
        total = row[2:14].sum()
        new.iloc[y, x] += total
    new = new.applymap(lambda x: min(round(x / 20), 255))
    return new

def project_and_save(df: pd.DataFrame, name: str):
    img = Image.fromarray(df.to_numpy(dtype=np.uint8), mode="L")
    img = img.resize((TARGET_WIDTH, TAGRET_HEIGHT), Image.Resampling.BICUBIC)
    img.save(name)

def main():
    if DO_TEMP:
        tmp = pd.read_csv("grid_10min_tmp.dat", dtype=float)
        tmp = convert_tmp(tmp)
        project_and_save(tmp, "tmp.png")
    else:
        print("Skipping temperature data.")
    if DO_PRE:
        pre = pd.read_csv("grid_10min_pre.dat", dtype=float)
        pre = convert_pre(pre)
        project_and_save(pre, "pre.png")
    else:
        print("Skipping precipitation data.")


if __name__ == "__main__":
    main()
