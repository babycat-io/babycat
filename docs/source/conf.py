"""Configuration file for the Sphinx documentation builder."""
from typing import List


# -- Core metadata --------------------------------------------------

project = 'Babycat'
copyright = '2021, Neocrym Records Inc.'
author = 'Neocrym Records Inc.'


# -- Sphinx extensions ----------------------------------------------

extensions = [
    # Renders Sphinx docs from JavaScript/TypeScript source code.
    "sphinx_js",

    # Renders Sphinx docs from Python docstrings.
    "sphinx.ext.autodoc",

    # Generates autodoc summaries.
    "sphinx.ext.autosummary",

    # Adds support for NumPy and Google-style docstrings.
    "sphinx.ext.napoleon",

    # Allows for linking to other Sphinx docs websites.
    "sphinx.ext.intersphinx",

    # Renders Sphinx docs from C docstrings.
    # Breathe runs Doxygen on C headers to generate Doxygen XML.
    # Breathe then renders the Doxygen XML as Sphinx.
    "breathe",

    # Add type hints.
    "sphinx_autodoc_typehints",

    # Generates Open Graph meta tags. Useful for Twitter/FB embed.
    "sphinxext.opengraph",

    # Generates content with clickable tabs.
    # Useful for showing code samples in multiple languages.
    "sphinx_inline_tabs",
]


# -- Mappings to other Sphinx documentation sites -------------------

intersphinx_mapping = {
    # Babycat's Python bindings target Python 3.6.
    "python": ("https://docs.python.org/3.6", None),
    # Babycat works with NumPy versions 1.16 or newer.
    "numpy": ("https://numpy.org/doc/1.16", None),
}


# -- Options for HTML output ----------------------------------------

templates_path = ['_templates']
html_theme = "neocrym_sphinx_theme"
html_static_path = ["_static"]
html_css_files = ["css/custom.css"]
html_show_sphinx = False
html_copy_source = False
html_show_source = False
html_theme_options = dict(
    light_logo="https://static.neocrym.com/images/babycat/v1/SVG/babycat-wordmark-on-transparent-black-text.svg",
    dark_logo="https://static.neocrym.com/images/babycat/v1/SVG/babycat-wordmark-on-transparent-white-text.svg",
    sidebar_hide_name=True,
)
add_module_names = False

# -- Options for syntax highlighting --------------------------------

pygments_style = "colorful"
pygments_dark_style = "fruity"


# -- Options for sphinxext.opengraph --------------------------------

ogp_image = "https://static.neocrym.com/images/babycat/v1/1x/babycat-body-icon-dark-social-media-cover--1x.png"

ogp_custom_meta_tags = [
    '<meta name="twitter:card" content="summary_large_image" />',
    '<meta property="twitter:image" content="https://static.neocrym.com/images/babycat/v1/1x/babycat-body-icon-dark-social-media-cover--1x.png" />',
]


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
    babycat=("../..",["babycat.h"]),
)

breathe_default_project = "babycat"

# Show the source code values for #define constants.
breathe_show_define_initializer = True

# Show the source code values for enums.
breathe_show_enumvalue_initializer = True


# Custom configuration
autosummary_generate = True
autosummary_imported_members = True
autosummary_generate_overwrite = True

autoclass_content = "both"
autodoc_inherit_docstrings = True
autodoc_docstring_signature = False

autodoc_default_options = {
    "members": True,
    "recursive": True,
    "undoc-members": True,
    "private-members": True,
    "special-members": None,
    "member-order": "bysource",
    "show-inheritance": True,
}


# -- Options for JavaScript/TypeScript output -----------------------

js_language = "javascript" # not TypeScript

js_source_path = "../../target/wasm/bundler/"

