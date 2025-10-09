# Configuration file for the Sphinx documentation builder
# See https://www.sphinx-doc.org/en/master/usage/configuration.html

from importlib.metadata import version, PackageNotFoundError
import sys
from pathlib import Path

# -- Path setup ---------------------------------------------------------------
# Add project root to sys.path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

# -- Project information ------------------------------------------------------
project = "causal-hub"
author = "Alessio Zanga"
copyright = f"2025, {author}"

try:
    release = version("causal-hub")
except PackageNotFoundError:
    release = "0.0.0"

# Use short X.Y version for display if needed
version = ".".join(release.split(".")[:2])

# -- General configuration ----------------------------------------------------
extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
    "sphinx.ext.autodoc.typehints",
    "sphinx.ext.intersphinx",
]

# Autodoc settings
autodoc_default_options = {
    "members": True,
    "undoc-members": False,
    "inherited-members": True,
    "show-inheritance": True,
    "special-members": "__call__,__init__,__new__",
    "private-members": False,
    "member-order": "bysource",
    "typehints": "description",
    "autosummary": True,
}
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
    "numpy": ("https://numpy.org/doc/stable/", None),
    "pandas": ("https://pandas.pydata.org/docs/", None),
    "networkx": ("https://networkx.org/documentation/stable/", None),
}
nitpicky = True
templates_path = ["_templates"]
exclude_patterns = []

# -- HTML output options ------------------------------------------------------
html_theme = "pydata_sphinx_theme"
html_theme_options = {
    "logo": {"text": project},
    "show_prev_next": False,
    "navigation_with_keys": True,
    "show_toc_level": 3,
}

html_static_path = ["_static"]

# -- Automatically generate recursive autosummary for all submodules ----------


def recursive_submodules(package_dir: Path):
    """Return all submodules by scanning .pyi files recursively."""
    modules = []
    for pyi in package_dir.rglob("*.pyi"):
        relative = pyi.relative_to(package_dir.parent).with_suffix("")
        modules.append(".".join(relative.parts))
    return sorted(modules)


package_path = Path(__file__).parents[2] / "causal_hub"
all_modules = recursive_submodules(package_path)

# Write a master autosummary.rst
autosummary_index = Path(__file__).parent / "autosummary.rst"
with open(autosummary_index, "w", encoding="utf-8") as f:
    f.write("Module Reference\n")
    f.write("================\n\n")
    f.write(".. toctree::\n")
    f.write("   :maxdepth: 2\n")
    f.write("   :caption: Contents:\n\n")
    f.write(".. autosummary::\n")
    f.write("   :toctree: _autosummary\n")
    f.write("   :recursive:\n\n")
    for mod in all_modules:
        f.write(f"   {mod}\n")
