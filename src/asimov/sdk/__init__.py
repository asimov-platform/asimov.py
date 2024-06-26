# This is free and unencumbered software released into the public domain.

"""The Asimov Software Development Kit (SDK) for Python."""

import sys

assert sys.version_info >= (3, 9), "The Asimov SDK for Python requires Python 3.9+"

from .lib import *
from .util import *

__all__ = [
    'lib',
    'util',
]
