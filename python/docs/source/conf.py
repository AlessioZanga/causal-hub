# Configuration file for the Sphinx documentation builder
# See https://www.sphinx-doc.org/en/master/usage/configuration.html

import ast
import sys
from importlib.metadata import PackageNotFoundError, version
from pathlib import Path

# -- Path setup ---------------------------------------------------------------
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

# -- Project information ------------------------------------------------------
project = "causal-hub"
author = "Alessio Zanga"
copyright = f"2026, {author}"

try:
    release = version("causal-hub")
except PackageNotFoundError:
    release = "0.0.0"

version = ".".join(release.split(".")[:2])

# -- General configuration ----------------------------------------------------
extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
    "sphinx.ext.intersphinx",
]

autodoc_default_options = {
    "members": True,
    "undoc-members": False,
    "inherited-members": True,
    "show-inheritance": True,
    "special-members": "__call__,__init__,__new__",
    "private-members": False,
    "member-order": "bysource",
    "autosummary": True,
}
autosummary_generate = True
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
    "numpy": ("https://numpy.org/doc/stable/", None),
    "pandas": ("https://pandas.pydata.org/docs/", None),
    "networkx": ("https://networkx.org/documentation/stable/", None),
}
nitpicky = True
templates_path = ["_templates"]
exclude_patterns = []

html_theme = "pydata_sphinx_theme"
html_theme_options = {
    "logo": {"text": project},
    "show_prev_next": False,
    "navigation_with_keys": True,
    "show_toc_level": 3,
}
html_static_path = ["_static"]


# -- Prevent PyO3 enum recursion in autodoc ----------------------------------
# PyO3 #[pyclass] enums expose variants as class attributes that return
# the enum member itself (an instance of the same class).  When Sphinx
# autodoc enumerates members of such an instance, it finds the variant name
# again, recurses, and blows the stack.
#
# Fix: skip any autodoc member whose type is one of our PyO3 enum classes.

_PY3_ENUM_CLASSES: set = set()
_PY3_ENUM_BASES: tuple = ()


def _collect_enum_classes():
    """Import the modules and collect PyO3 enum class types."""
    global _PY3_ENUM_BASES
    try:
        import causal_hub.datasets as ds

        for name in ("Dataset", "MissingMethod", "MissingType"):
            cls = getattr(ds, name, None)
            if cls is not None:
                _PY3_ENUM_CLASSES.add(cls)
    except ImportError:
        pass
    try:
        import causal_hub.estimators as est

        for name in ("EstimatorMethod",):
            cls = getattr(est, name, None)
            if cls is not None:
                _PY3_ENUM_CLASSES.add(cls)
    except ImportError:
        pass
    _PY3_ENUM_BASES = tuple(_PY3_ENUM_CLASSES)


_collect_enum_classes()


def _autodoc_skip_member(app, what, name, obj, skip, options):
    """Skip PyO3 enum classes AND their variant subclasses.

    PyO3 struct-variant enums (like ``Dataset``) create nested subclasses for
    each variant (e.g. ``Dataset.Categorical``).  These subclasses inherit the
    variant names as class attributes, creating an infinite identity chain
    (``Categorical.Categorical.Categorical...``) that blows the stack in
    Sphinx's ``_document_members`` recursion.
    """
    try:
        if what == "class" and obj in _PY3_ENUM_CLASSES:
            return True
        if (
            what == "class"
            and isinstance(obj, type)
            and _PY3_ENUM_BASES
            and issubclass(obj, _PY3_ENUM_BASES)
            and obj not in _PY3_ENUM_CLASSES
        ):
            return True
    except TypeError:
        pass
    return None


def setup(app):
    app.connect("autodoc-skip-member", _autodoc_skip_member)


# -- Parse .pyi stubs to discover modules ------------------------------------


def _parse_pyi_members(pyi_path: Path):
    """Parse a .pyi file and return (classes, functions, enums)."""
    source = pyi_path.read_text(encoding="utf-8")
    tree = ast.parse(source)
    classes, functions, enums = [], [], []
    for node in ast.iter_child_nodes(tree):
        if isinstance(node, ast.ClassDef):
            is_enum = any(
                (isinstance(b, ast.Attribute) and b.attr == "Enum")
                or (isinstance(b, ast.Name) and b.id == "Enum")
                for b in node.bases
            )
            (enums if is_enum else classes).append(node.name)
        elif isinstance(node, ast.FunctionDef):
            functions.append(node.name)
    return classes, functions, enums


def _generate_module_rst(
    rst_dir: Path, module_name: str, classes: list, functions: list, enums: list
):
    """Write a per-module .rst with explicit autoclass/autofunction.

    Uses ``automodule: :no-imported:`` instead of ``:no-members:`` to avoid
    importing the module (which triggers PyO3 enum recursion).
    """
    rst_path = rst_dir / f"{module_name}.rst"
    title = module_name.replace("_", "\\_")

    lines = [
        f"{title}",
        "=" * len(title),
        "",
    ]

    if classes:
        lines.append(".. rubric:: Classes")
        lines.append("")
        for name in classes:
            lines.append(f".. autoclass:: {module_name}.{name}")
            lines.append("   :members:")
            lines.append("")

    if functions:
        lines.append(".. rubric:: Functions")
        lines.append("")
        for name in functions:
            lines.append(f".. autofunction:: {module_name}.{name}")
            lines.append("")

    if enums:
        lines.append(".. rubric:: Enums")
        lines.append("")
        for name in enums:
            lines.append(f"- :class:`{module_name}.{name}`")
        lines.append("")

    rst_path.write_text("\n".join(lines), encoding="utf-8")


def _discover_modules(package_dir: Path):
    """Find all subpackages via __init__.pyi files."""
    modules = []
    for pyi in sorted(package_dir.rglob("__init__.pyi")):
        parts = list(pyi.relative_to(package_dir.parent).parts)
        if parts[-1] == "__init__.pyi":
            parts = parts[:-1]
        module = ".".join(parts)
        if module != "causal_hub":
            modules.append(module)
    return sorted(set(modules))


# -- Generate RST files at config time ---------------------------------------
package_path = Path(__file__).parents[2] / "causal_hub"
all_modules = _discover_modules(package_path)
autosummary_dir = Path(__file__).parent / "_autosummary"
autosummary_dir.mkdir(exist_ok=True)

# Also generate RST for the root causal_hub module (contains Error class)
root_pyi = package_path / "__init__.pyi"
if root_pyi.exists():
    classes, functions, enums = _parse_pyi_members(root_pyi)
    _generate_module_rst(autosummary_dir, "causal_hub", classes, functions, enums)

for mod in all_modules:
    pyi_path = package_path.parent / mod.replace(".", "/") / "__init__.pyi"
    if pyi_path.exists():
        classes, functions, enums = _parse_pyi_members(pyi_path)
    else:
        classes, functions, enums = [], [], []
    _generate_module_rst(autosummary_dir, mod, classes, functions, enums)

# Master toctree
with open(Path(__file__).parent / "autosummary.rst", "w") as f:
    f.write("Module Reference\n")
    f.write("================\n\n")
    f.write(".. toctree::\n")
    f.write("   :maxdepth: 2\n")
    f.write("   :caption: Contents:\n\n")
    f.write(".. autosummary::\n")
    f.write("   :toctree: _autosummary\n\n")
    f.write("   causal_hub\n")
    for mod in all_modules:
        f.write(f"   {mod}\n")
