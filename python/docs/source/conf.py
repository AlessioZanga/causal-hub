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
]

# Autodoc settings
autodoc_default_options = {
    "members": True,
    "undoc-members": False,
    "inherited-members": True,
    "show-inheritance": True,
    "special-members": "__init__,__call__",
    "private-members": False,
    "member-order": "bysource",
    "typehints": "description",
    "autosummary": True,
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
