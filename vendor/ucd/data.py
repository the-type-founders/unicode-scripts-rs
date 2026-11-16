# /// script
# requires-python = ">=3.9"
# ///

import json
import sys
from urllib.request import urlopen

# Update this to upgrade to a new release of the Unicode standard.
VERSION = "18.0.0"

ROOT = f"https://www.unicode.org/Public/{VERSION}/ucd"


def read(name):
    with urlopen(f"{ROOT}/{name}") as response:
        for line in response.read().decode("utf-8").splitlines():
            # Remove the comment if any.
            line = line.split("#")[0].strip()
            if line:
                yield [field.strip() for field in line.split(";")]


def merge(ranges):
    result = []
    for start, end in sorted(ranges):
        if result and result[-1][1] == start:
            result[-1][1] = end
        else:
            result.append([start, end])
    return result


# Map the long names of scripts onto their ISO 15924 codes. The first alias is the code; the ones
# after the long name, if any, are deprecated.
codes = {}
for property, code, name, *_ in read("PropertyValueAliases.txt"):
    if property == "sc":
        codes[name] = code

scripts = {}
for range, name in read("Scripts.txt"):
    start, _, end = range.partition("..")
    start = int(start, 16)
    # The upper bound is exclusive.
    end = int(end or range, 16) + 1
    scripts.setdefault(name, []).append([start, end])
scripts = {name: merge(ranges) for name, ranges in scripts.items()}

data = [{"code": codes[name], "name": name, "ranges": ranges} for name, ranges in sorted(scripts.items())]
json.dump(data, sys.stdout, indent=2)
sys.stdout.write("\n")
