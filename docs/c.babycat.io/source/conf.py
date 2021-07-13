"""Configuration file for the Sphinx documentation builder."""
from typing import List

# -- Project information -----------------------------------------------------

project = "Babycat C"
copyright = "2021, Neocrym Records Inc."  # pylint: disable=redefined-builtin
author = "Neocrym Records Inc."


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    # `breathe` generates Doxygen XML and renders it with Sphinx.
    "breathe",
    "sphinx_inline_tabs",
    # `sphinx-multiversion` is used to render multiple multiple Git
    # branches and tags of the same documentation website.
    "sphinx_multiversion",
    "sphinxext.opengraph",
]

# Add any paths that contain templates here, relative to this directory.
templates_path = ["_templates"]

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns: List[str] = []


# -- Options for HTML output -------------------------------------------------

html_theme = "neocrym_sphinx_theme"
html_show_sphinx = False
html_copy_source = False
html_show_source = False
html_css_files = ["css/custom.css"]

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
html_static_path = ["_static"]

# Specify C as our default Sphinx documentation domain.
primary_domain = "c"

# Specify that code samples should be highlighted as C code by default.
highlight_language = "c"

pygments_style = "colorful"

# We would normally use `fruity` as the pygments_dark_style,
# but its choice of colors are really bad for the C language.
# The `inkpot` syntax highlighter is much better, although
# we fix it up a bit in our custom.css.
pygments_dark_style = "inkpot"

# -- Options for Breathe/Doxygen configuration -------------------------------

breathe_doxygen_config_options = dict(
    SEPARATE_MEMBER_PAGES="NO",
    # Tell Doxygen to skip trying to render images using the
    # `dot` software package.
    CLASS_DIAGRAMS="NO",
    HAVE_DOT="NO",
)

# Doxygen does not need to generate separate member pages
# when we are using breathe and Sphinx.
breathe_separate_member_pages = False

# We tell Doxygen to load our babycat.h.
breathe_projects_source = dict(
    babycat=("../../..",["babycat.h"]),
)

breathe_default_project = "babycat"

# Show the source code values for #define constants.
breathe_show_define_initializer = True

# Show the source code values for enums.
breathe_show_enumvalue_initializer = True
