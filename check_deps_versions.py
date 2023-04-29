import tomli
import requests

deps_version = {}
deps_latest_version = {}
deps_section_key = 'dependencies'

def get_latest_version(depend_name):
    url = "https://crates.io/api/v1/crates/%s" % depend_name
    r = requests.get(url=url)
    return r.json()['crate']['max_stable_version']

with open("Cargo.toml", "rb") as f:
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
        msg = "found outdated version for crate [%s], current_version=%s, latest_version=%s" % (key, current_version, last_version)
        print(msg)

if outdated_versions == False:
    print("outdated versions not found")
