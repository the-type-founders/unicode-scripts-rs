# /// script
# requires-python = ">=3.9"
# ///

import json
import sys
from urllib.request import urlopen

# Update this to upgrade to a new release of the Unicode standard.
VERSION = "18.0.0"

ROOT = f"https://www.unicode.org/Public/{VERSION}/ucd"


# The marker of a line giving the value of the codepoints that are not listed.
MISSING = "# @missing:"


def read(name):
    with urlopen(f"{ROOT}/{name}") as response:
        for line in response.read().decode("utf-8").splitlines():
            missing = line.startswith(MISSING)
            if missing:
                line = line[len(MISSING) :]
            # Remove the comment if any.
            line = line.split("#")[0].strip()
            if line:
                yield missing, [field.strip() for field in line.split(";")]


def parse(range):
    start, _, end = range.partition("..")
    # The upper bound is exclusive.
    return [int(start, 16), int(end or start, 16) + 1]


def merge(ranges):
    result = []
    for start, end in sorted(ranges):
        if result and result[-1][1] == start:
            result[-1][1] = end
        else:
            result.append([start, end])
    return result


def subtract(range, ranges):
    result = []
    cursor, limit = range
    for start, end in merge(ranges):
        if cursor < start:
            result.append([cursor, min(start, limit)])
        cursor = max(cursor, end)
    if cursor < limit:
        result.append([cursor, limit])
    return result


# Map the long names of scripts onto their ISO 15924 codes. The first alias is the code; the ones
# after the long name, if any, are deprecated.
codes = {}
for missing, (property, code, name, *_) in read("PropertyValueAliases.txt"):
    if not missing and property == "sc":
        codes[name] = code

scripts = {}
defaults = []
for missing, (range, name) in read("Scripts.txt"):
    if missing:
        defaults.append((parse(range), name))
    else:
        scripts.setdefault(name, []).append(parse(range))
# Assign the codepoints that are not listed.
listed = [range for ranges in scripts.values() for range in ranges]
for range, name in defaults:
    scripts.setdefault(name, []).extend(subtract(range, listed))
scripts = {name: merge(ranges) for name, ranges in scripts.items()}

data = [{"code": codes[name], "name": name, "ranges": ranges} for name, ranges in sorted(scripts.items())]
json.dump(data, sys.stdout, indent=2)
sys.stdout.write("\n")
