"""Fetch China admin divisions from jsdelivr CDN (single file: pca-code.json).

Outputs data/regions.json as a flat array:
  {"code": "110000", "name": "北京市", "level": "province"}
  {"code": "110100", "name": "市辖区", "level": "city", "parent_code": "110000"}
  {"code": "110101", "name": "东城区", "level": "district", "parent_code": "110100"}
"""

import json
import os
import urllib.request

URL = "https://cdn.jsdelivr.net/gh/modood/Administrative-divisions-of-China@master/dist/pca-code.json"

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(SCRIPT_DIR, "..", "data", "regions.json")


def pad_code(code: str) -> str:
    """Province codes are 2-digit, need to pad to 6-digit for consistency.
       City codes are 4-digit, district codes are 6-digit."""
    if len(code) == 2:
        return code + "0000"
    elif len(code) == 4:
        return code + "00"
    return code


def flatten(nodes: list[dict], parent: str | None, result: list[dict]):
    for node in nodes:
        code = pad_code(node["code"])
        level = (
            "province" if len(node["code"]) == 2
            else "city" if len(node["code"]) == 4
            else "district"
        )
        entry: dict = {"code": code, "name": node["name"], "level": level}
        if parent:
            entry["parent_code"] = parent
        result.append(entry)
        if "children" in node:
            flatten(node["children"], code, result)


def main():
    print(f"Fetching {URL} ...")
    req = urllib.request.Request(URL, headers={"User-Agent": "fetch-regions/1.0"})
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.loads(resp.read().decode("utf-8"))

    result: list[dict] = []
    flatten(data, None, result)

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(result, f, ensure_ascii=False)

    by_level = {"province": 0, "city": 0, "district": 0}
    for r in result:
        by_level[r["level"]] += 1
    print(
        f"Written {len(result)} regions "
        f"({by_level['province']} provinces, {by_level['city']} cities, "
        f"{by_level['district']} districts) → {OUT}"
    )


if __name__ == "__main__":
    main()
