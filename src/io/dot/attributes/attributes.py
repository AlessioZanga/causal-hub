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


rs_attrs = ""

rs_attrs += "use std::hash::{Hash, Hasher};\n"
rs_attrs += "\n"
rs_attrs += "use crate::types::FxIndexSet;\n"
rs_attrs += "\n"
rs_attrs += "/// Quote string if necessary.\n"
rs_attrs += "fn quote(s: &str) -> String {\n"
rs_attrs += "    // Check if quoted and needs quoting.\n"
rs_attrs += "    if !(s.starts_with(\"\\\"\") && s.ends_with(\"\\\"\")) && s.contains(\" \") {\n"
rs_attrs += "        // Add quoting to given string.\n"
rs_attrs += "        return format!(\"\\\"{s}\\\"\");\n"
rs_attrs += "    }\n"
rs_attrs += "\n"
rs_attrs += "    s.into()\n"
rs_attrs += "}\n"
rs_attrs += "\n"
rs_attrs += "/// Attribute enumerator.\n"
rs_attrs += "#[derive(Clone, Debug)]\n"
rs_attrs += "pub enum Attribute {\n"
for (cm_k, k, v) in sorted((sc2cm(k), k, v) for (k, v) in attrs.items()):
    rs_attrs += f"    /// {v['Description']} <a href=\"https://graphviz.org/docs/attrs/{k.removeprefix('_')}/\" target=\"_blank\">Read more</a>.\n"
    rs_attrs += f"    {cm_k}(String),\n"
rs_attrs += "}\n"
rs_attrs += "\n"
rs_attrs += "impl PartialEq for Attribute {\n"
rs_attrs += "    fn eq(&self, other: &Self) -> bool {\n"
rs_attrs += "        // Compare attributes based on their discriminant.\n"
rs_attrs += "        std::mem::discriminant(self).eq(&std::mem::discriminant(other))\n"
rs_attrs += "    }\n"
rs_attrs += "}\n"
rs_attrs += "\n"
rs_attrs += "impl Eq for Attribute {}\n"
rs_attrs += "\n"
rs_attrs += "impl Hash for Attribute {\n"
rs_attrs += "    fn hash<H: Hasher>(&self, state: &mut H) {\n"
rs_attrs += "        // Hash attributes based on their discriminant.\n"
rs_attrs += "        std::mem::discriminant(self).hash(state);\n"
rs_attrs += "    }\n"
rs_attrs += "}\n"
rs_attrs += "\n"
rs_attrs += "impl From<Attribute> for (String, String) {\n"
rs_attrs += "    fn from(attribute: Attribute) -> Self {\n"
rs_attrs += "        let (key, value) = match attribute {\n"
for (cm_k, k) in sorted((sc2cm(k), k) for k in attrs.keys()):
    rs_attrs += f"            Attribute::{cm_k}(x) => (\"{k}\", x),\n"
rs_attrs += "        };\n"
rs_attrs += "\n"
rs_attrs += "        (key.into(), value)\n"
rs_attrs += "    }\n"
rs_attrs += "}\n"
rs_attrs += "\n"

for v in attrs.values():
    v["Used By"] = v["Used By"].split(", ")

used_by = defaultdict(OrderedSet)
for (k, v) in attrs.items():
    for u in v["Used By"]:
        u = u.replace("Node", "Vertex")
        used_by[u].add(k)

rs_used_by = ""

for (w, (k, v)) in enumerate(used_by.items()):
    rs_used_by += f"/// {k.removesuffix('s')} attributes.\n"
    rs_used_by += "#[derive(Clone, Debug, Default)]\n"
    rs_used_by += f"pub struct {k.removesuffix('s')}Attributes" + " {\n"
    rs_used_by += "    attributes: FxIndexSet<Attribute>,\n"
    rs_used_by += "}\n"
    rs_used_by += "\n"
    rs_used_by += f"impl {k.removesuffix('s')}Attributes" + " {\n"
    rs_used_by += "    /// Set attribute from `key` and `value` raw parts. Returns whether the attribute was newly set.\n"
    rs_used_by += "    ///\n"
    rs_used_by += "    /// # Panics\n"
    rs_used_by += "    ///\n"
    rs_used_by += "    /// Key is not valid for this attributes set. <a href=\"https://graphviz.org/doc/info/attrs.html#h:uses\" target=\"_blank\">Read more</a>.\n"
    rs_used_by += "    ///\n"
    rs_used_by += "    pub fn insert_raw_parts(&mut self, key: &str, value: &str) -> bool {\n"
    rs_used_by += "        let value = quote(value);\n"
    rs_used_by += "        let item = match key {\n"
    for i in v:
        rs_used_by += f"            \"{i}\" => Attribute::{sc2cm(i)}(value),\n"
    rs_used_by += "            _ => panic!(\"Invalid attribute key `{key}` for " + \
        f"{k.removesuffix('s')}Attributes\"),\n"
    rs_used_by += "        };\n"
    rs_used_by += "\n"
    rs_used_by += "        self.attributes.replace(item).is_none()\n"
    rs_used_by += "    }\n"
    rs_used_by += "\n"
    rs_used_by += "    /// Get attributes length.\n"
    rs_used_by += "    #[inline]\n"
    rs_used_by += "    pub fn len(&self) -> usize {\n"
    rs_used_by += "        self.attributes.len()\n"
    rs_used_by += "    }\n"
    rs_used_by += "\n"
    rs_used_by += "    /// Check if attributes is empty.\n"
    rs_used_by += "    #[inline]\n"
    rs_used_by += "    pub fn is_empty(&self) -> bool {\n"
    rs_used_by += "        self.attributes.is_empty()\n"
    rs_used_by += "    }\n"
    rs_used_by += "\n"
    for (j, i) in enumerate(v):
        rs_used_by += f"    /// Set [`Attribute::{sc2cm(i)}`] attribute. Returns whether the attribute was newly set.\n"
        rs_used_by += "    #[inline]\n"
        rs_used_by += f"    pub fn set_{i.removeprefix('_').lower()}(&mut self, s: &str) -> bool" + " {\n"
        rs_used_by += "        // Initialize new item for insertion or replacement.\n"
        rs_used_by += f"        let item = Attribute::{sc2cm(i)}(quote(s));\n"
        rs_used_by += "\n"
        rs_used_by += "        self.attributes.replace(item).is_none()\n"
        rs_used_by += "    }\n"
        rs_used_by += "\n"
        rs_used_by += f"    /// Unset [`Attribute::{sc2cm(i)}`] attribute. Returns whether the attribute was set.\n"
        rs_used_by += "    #[inline]\n"
        rs_used_by += f"    pub fn unset_{i.removeprefix('_').lower()}(&mut self) -> bool" + " {\n"
        rs_used_by += "        // Allocate item placeholder for removal.\n"
        rs_used_by += f"        let item = Attribute::{sc2cm(i)}(String::new());\n"
        rs_used_by += "\n"
        rs_used_by += "        self.attributes.remove(&item)\n"
        rs_used_by += "    }\n"
        if j != len(v) - 1:
            rs_used_by += "\n"
    rs_used_by += "}\n"
    rs_used_by += "\n"
    rs_used_by += f"impl IntoIterator for {k.removesuffix('s')}Attributes" + \
        " {\n"
    rs_used_by += "    type Item = Attribute;\n"
    rs_used_by += "\n"
    rs_used_by += "    type IntoIter = indexmap::set::IntoIter<Attribute>;\n"
    rs_used_by += "\n"
    rs_used_by += "    #[inline]\n"
    rs_used_by += "    fn into_iter(self) -> Self::IntoIter {\n"
    rs_used_by += "        self.attributes.into_iter()\n"
    rs_used_by += "    }\n"
    rs_used_by += "}\n"
    if w != len(used_by) - 1:
        rs_used_by += "\n"

with open("mod.rs", "w") as file:
    file.write(
        f"// Automatically generated on: {datetime.now()} .\n\n" + rs_attrs + rs_used_by)
