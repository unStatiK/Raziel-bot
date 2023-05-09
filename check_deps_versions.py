import tomli
import requests
import sys

USER_AGENT = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0"
NORMAL_RESULT_PREFIX = "not_outdated"
OUTDATED_RESULT_PREFIX = "found_outdated"

deps_version = {}
deps_latest_version = {}
deps_section_key = 'dependencies'

def get_latest_version(depend_name):
    url = "https://crates.io/api/v1/crates/%s" % depend_name
    r = requests.get(url=url,headers={"User-Agent": USER_AGENT, "Upgrade-Insecure-Requests": "1"})
    if r.status_code == 200:
        return r.json()['crate']['max_stable_version']
    return ""    

if len(sys.argv) != 2:
    exit()

cargo_file_path = sys.argv[1]

with open(cargo_file_path, "rb") as f:
    data = tomli.load(f)
    for key in data[deps_section_key]:
        item = data[deps_section_key][key]
        if type(item) is dict:
            deps_version[key] = item['version']
        if type(item) is str:
            deps_version[key] = item
        deps_latest_version[key] = get_latest_version(key)

outdated_versions = False
for key in deps_version:
    current_version = str(deps_version[key]).lower()
    last_version = str(deps_latest_version[key]).lower()
    if current_version != last_version:
        outdated_versions = True
        msg = "%s: found outdated version for crate [%s], current_version=%s, latest_version=%s" % (OUTDATED_RESULT_PREFIX, key, current_version, last_version)
        print(msg)

if outdated_versions == False:
    print("%s: outdated versions not found" % NORMAL_RESULT_PREFIX)
