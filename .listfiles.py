#!/usr/bin/env python3
"""
A cross-platform command for listing files in a directory.

You can pass an arbitrary number of paths, and ``listfiles`` will
list all of the files and directories under each path.

If you do not pass any paths, then ``listfiles`` will list
all of the files and directories in your current working directory.

Examples:
    python3 .listfiles.py src tests

"""
import os
import sys

# Calculate the absolute paths to scan, based on the arguments
# provided to this script.
paths = [
    os.path.abspath(path)
    for path in sys.argv[1:]
]


# Remove redundant paths.
if paths:
    paths = sorted(set(paths))
else:
    paths = [os.getcwd()]


# Iterate over each path passed to this script...
for root in paths:
    # ...and iterate over the directory tree in each path.
    for prefix, dirnames, filenames in os.walk(
        root,
        topdown=True,
        followlinks=True,
    ):
        for dirname in dirnames:
            print(os.path.join(prefix, dirname))
        for filename in filenames:
            print(os.path.join(prefix, filename))
