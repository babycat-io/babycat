"""Configuration file for the Sphinx documentation builder."""
from typing import List

#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#
# import os
# import sys
# sys.path.insert(0, os.path.abspath('.'))


# -- Project information -----------------------------------------------------

project = "Babycat"
copyright = "2021, Neocrym Records Inc."  # pylint: disable=redefined-builtin
author = "Neocrym Records Inc."


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = ["sphinx_inline_tabs"]

# Add any paths that contain templates here, relative to this directory.
templates_path = ["_templates"]

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns: List[str] = []


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = "furo"
html_show_sphinx = False
html_copy_source = False
html_show_source = False
html_css_files = ["css/custom.css"]
html_theme_options = {
    "dark_css_variables": {
        "color-foreground-primary": "#c5c5c5",
        "color-background-primary": "#131416",
        "color-background-secondary": "#1a1c1e",
        "color-brand-primary": "#5aa8ed",
        "color-brand-content": "#5aa8ed",
    },
}
# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
html_static_path = ["_static"]

pygments_style = "colorful"
pygments_dark_style = "fruity"
