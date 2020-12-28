import json, sys

if len(sys.argv) < 2:
    print('update_package_json.py missing args')
    sys.exit(1)

file_location = sys.argv[1]

json_str = ''
with open(file_location, 'r') as f:
    json_str = ''
    for line in f.readlines():
        json_str += line
 
pkg = json.loads(json_str)
pkg['name'] = 'sejong-buffer'
if 'files' in pkg:
    del pkg['files']
new_json_str = json.dumps(pkg, indent=2)

with open(file_location, 'w') as f:
    f.write(new_json_str)