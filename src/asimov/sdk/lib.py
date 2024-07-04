# This is free and unencumbered software released into the public domain.

"""The Asimov Software Development Kit (SDK) for Python."""

import os
import platform

from ctypes import *

MACOS_APP_PATH = 'Applications/Asimov.app'
MACOS_APP_SUBPATH = 'Contents/Frameworks/AsimovSDK.framework/AsimovSDK'
MACOS_SDK_DYLIB = 'libAsimovSDK.dylib'

def load_library() -> CDLL:
    for path in paths():
        try:
            return cdll.LoadLibrary(path)
        except OSError as err:
            #print(err)
            pass
    raise ModuleNotFoundError("Unable to find the Asimov SDK binaries")

def paths() -> list[str]:
    result = []
    sys = platform.system()

    if sys == 'Darwin': # macOS
        if (path := os.getenv('ASIMOV_SDK')) is not None:
            result.append(path)
        if (path := os.getenv('ASIMOV_APP')) is not None:
            result.append(os.path.join(path, MACOS_APP_SUBPATH))
        if (path := os.getenv('HOME')) is not None:
            result.append(os.path.join(path, MACOS_APP_PATH, MACOS_APP_SUBPATH))
        result.append(os.path.join('/', MACOS_APP_PATH, MACOS_APP_SUBPATH))
        result.append(MACOS_SDK_DYLIB)

    elif sys == 'Linux':
        result.append('libAsimovSDK.so')

    elif sys == 'Windows':
        result.append('AsimovSDK.dll')

    return list(dict.fromkeys(result)) # remove duplicates
