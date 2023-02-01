# !pip install pandas lxml sortedcollections

import requests
import pandas as pd

from collections import defaultdict
from datetime import datetime
from sortedcollections import OrderedSet

attrs = "https://graphviz.org/doc/info/attrs.html"
attrs = requests.get(attrs).content
attrs = pd.read_html(attrs)[0]

assert ("Description, notes" in attrs.columns)
attrs.rename(columns={"Description, notes": "Description"}, inplace=True)

attrs = attrs.to_dict("records")
attrs = {a["Name"]: a for a in attrs}


# Snake Case to Camel Case.
def sc2cm(s):
    return "".join([s.title() for s in s.split("_")])


for v in attrs.values():
    v["Used By"] = v["Used By"].split(", ")

used_by = defaultdict(OrderedSet)
for (k, v) in attrs.items():
    for u in v["Used By"]:
        u = u.replace("Node", "Vertex")
        used_by[u].add(k)

rs_used_by = ""

for (w, (k, v)) in enumerate(used_by.items()):
    rs_used_by += f"mod {k.removesuffix('s').lower()}_attributes" + " {\n"
    rs_used_by += "\n"
    rs_used_by += f"    use causal_hub::io::dot::attributes::{k.removesuffix('s')}Attributes;\n"
    rs_used_by += "\n"
    for (j, i) in enumerate(v):
        rs_used_by += "    #[test]\n"
        rs_used_by += f"    fn {i.removeprefix('_').lower()}()" + " {\n"
        rs_used_by += f"        let mut attributes: {k.removesuffix('s')}Attributes = Default::default();\n"
        rs_used_by += f"        assert!(!attributes.unset_{i.removeprefix('_').lower()}());\n"
        rs_used_by += f"        assert!(attributes.set_{i.removeprefix('_').lower()}(\"TEST\"));\n"
        rs_used_by += f"        assert!(attributes.unset_{i.removeprefix('_').lower()}());\n"
        rs_used_by += "    }\n"
        if j!= len(v) - 1:
            rs_used_by += "\n"
    rs_used_by += "}\n"
    if w != len(used_by) - 1:
        rs_used_by += "\n"

with open("attributes.rs", "w") as file:
    file.write(
        f"// Automatically generated on: {datetime.now()} .\n\n" + rs_used_by)
