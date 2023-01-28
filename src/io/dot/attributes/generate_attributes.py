# !pip install pandas lxml sortedcollections

import os
import requests
import pandas as pd

from collections import defaultdict
from datetime import datetime
from sortedcollections import OrderedSet

attrs = "https://graphviz.org/doc/info/attrs.html"
attrs = requests.get(attrs).content
attrs = pd.read_html(attrs)[0]

assert("Description, notes" in attrs.columns)
attrs.rename(columns = { "Description, notes": "Description" }, inplace=True)

attrs = attrs.to_dict("records")
attrs = { a["Name"]: a for a in attrs }


# Snake Case to Camel Case.
def sc2cm(s):
    return "".join([s.title() for s in s.split("_")])


rs_attrs = ""

for (k, v) in attrs.items():
    rs_attrs += f"/// {v['Description']}\n"
    rs_attrs += "#[derive(Clone, Debug)]\n"
    rs_attrs += f"struct {sc2cm(k)}(pub(crate) String);\n"
    rs_attrs += "\n"
    rs_attrs += f"impl {sc2cm(k)}" + " {\n"
    rs_attrs += "    pub fn new(s: &str) -> Self {\n"
    path = f"./validate_{k.removeprefix('_').lower()}.in"
    if not os.path.exists(path):
        with open(path, "w") as file:
            file.write("{\n\t// FIXME: Validate input.\n\ts.into()\n}\n")
    rs_attrs += f"        Self(include!(\"{path}\"))\n"
    rs_attrs += "    }\n"
    rs_attrs += "}\n"
    rs_attrs += "\n"
    rs_attrs += f"impl std::fmt::Display for {sc2cm(v['Name'])}" + " {\n"
    rs_attrs += "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n"
    rs_attrs += f"        write!(f, \"{v['Name']}" + " = \\\"{}\\\"\", self.0)\n"
    rs_attrs += "    }\n"
    rs_attrs += "}\n"
    rs_attrs += "\n"

for v in attrs.values():
    v["Used By"] = v["Used By"].split(", ")

used_by = defaultdict(OrderedSet)
for (k, v) in attrs.items():
    for u in v["Used By"]:
        used_by[u].add(k)

rs_used_by = ""

for (w, (k, v)) in enumerate(used_by.items()):
    rs_used_by += f"/// {k.removesuffix('s')} attributes.\n"
    rs_used_by += "#[derive(Clone, Debug, Default)]\n"
    rs_used_by += f"pub struct {k.removesuffix('s')}Attrs" + " {\n"
    for i in v:
        rs_used_by += f"    {i.lower()}: Option<{sc2cm(i)}>,\n"
    rs_used_by += "}\n"
    rs_used_by += "\n"
    rs_used_by += f"impl {k.removesuffix('s')}Attrs" + " {\n"
    for (j, i) in enumerate(v):
        rs_used_by += f"    /// {attrs[i]['Description']} [Read more](https://graphviz.org/docs/attrs/{i.removeprefix('_')}/).\n"
        rs_used_by += f"    pub fn get_{i.removeprefix('_').lower()}(&self) -> Option<&str>" + " {\n"
        rs_used_by += f"        self.{i.lower()}.as_ref().map(|x| x.0.as_str())\n"
        rs_used_by += "    }\n"
        rs_used_by += "\n"
        rs_used_by += f"    /// Set `{i.lower()}` attribute. [Read more](https://graphviz.org/docs/attrs/{i.removeprefix('_')}/).\n"
        rs_used_by += f"    pub fn set_{i.removeprefix('_').lower()}(&mut self, s: &str)" + " {\n"
        rs_used_by += f"        self.{i.lower()} = Some({sc2cm(i)}::new(s));\n"
        rs_used_by += "    }\n"
        rs_used_by += "\n"
        rs_used_by += f"    /// Unset `{i.lower()}` attribute. [Read more](https://graphviz.org/docs/attrs/{i.removeprefix('_')}/).\n"
        rs_used_by += f"    pub fn unset_{i.removeprefix('_').lower()}(&mut self)" + " {\n"
        rs_used_by += f"        self.{i.lower()} = None;\n"
        rs_used_by += "    }\n"
        if j != len(v) - 1:
            rs_used_by += "\n"
    rs_used_by += "}\n"
    rs_used_by += "\n"
    rs_used_by += f"impl IntoIterator for {k.removesuffix('s')}Attrs" + " {\n"
    rs_used_by += "    type Item = String;\n"
    rs_used_by += "\n"
    rs_used_by += f"    type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<Self::Item>, {len(v)}>>;\n"
    rs_used_by += "\n"
    rs_used_by += "    fn into_iter(self) -> Self::IntoIter {\n"
    rs_used_by += "        [\n"
    for i in v:
        rs_used_by += f"            self.{i.lower()}.map(|x| x.to_string()),\n"
    rs_used_by += "        ]\n"
    rs_used_by += "        .into_iter()\n"
    rs_used_by += "        .flatten()\n"
    rs_used_by += "    }\n"
    rs_used_by += "}\n"
    if w != len(used_by) - 1:
        rs_used_by += "\n"

with open("mod.rs", "w") as file:
    file.write(f"// Automatically generated on: {datetime.now()} .\n\n" + rs_attrs + rs_used_by)
