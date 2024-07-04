# This is free and unencumbered software released into the public domain.

"""The Asimov Software Development Kit (SDK) for Python."""

from enum import IntEnum
from warnings import warn
from .lib import *

lib = load_library()

# typedef struct AsiInstance* AsiInstance
AsiInstance = c_void_p

# typedef uint64_t AsiVersion
AsiVersion = c_uint64

# typedef enum AsiResult { ... } AsiResult
class AsiResult(IntEnum):
    ASI_SUCCESS = 0
    ASI_TIMEOUT_EXPIRED = 1
    ASI_ERROR_NOT_IMPLEMENTED = -1
    ASI_ERROR_PRECONDITION_VIOLATED = -2
    ASI_ERROR_HOST_MEMORY_EXHAUSTED = -3
    ASI_ERROR_DEVICE_MEMORY_EXHAUSTED = -4

# AsiResult asiCreateInstance(const void* reserved, AsiInstance** instance)
try:
    asiCreateInstance = lib.asiCreateInstance
    asiCreateInstance.restype = AsiResult
    asiCreateInstance.argtypes = (c_void_p, c_void_p,)
except AttributeError as err:
    warn(err, ImportWarning)

# AsiResult asiDestroyInstance(AsiInstance* instance)
try:
    asiDestroyInstance = lib.asiDestroyInstance
    asiDestroyInstance.restype = AsiResult
    asiDestroyInstance.argtypes = (c_void_p,)
except AttributeError as err:
    warn(err, ImportWarning)

# const char* asiGetLicenseeString()
try:
    asiGetLicenseeString = lib.asiGetLicenseeString
    asiGetLicenseeString.restype = c_char_p
    asiGetLicenseeString.argtypes = ()
except AttributeError as err:
    warn(err, ImportWarning)

# uint64_t asiGetVersion()
try:
    asiGetVersion = lib.asiGetVersion
    asiGetVersion.restype = c_uint64
    asiGetVersion.argtypes = ()
except AttributeError as err:
    warn(err, ImportWarning)

# const char* asiGetVersionString()
try:
    asiGetVersionString = lib.asiGetVersionString
    asiGetVersionString.restype = c_char_p
    asiGetVersionString.argtypes = ()
except AttributeError as err:
    warn(err, ImportWarning)

# AsiResult asiInitLibrary(void* init_data, void* print_callback)
try:
    asiInitLibrary = lib.asiInitLibrary
    asiInitLibrary.restype = AsiResult
    asiInitLibrary.argtypes = (c_void_p, c_void_p,)
except AttributeError as err:
    warn(err, ImportWarning)

__all__ = [
    'AsiInstance',
    'AsiVersion',
    'AsiResult',
    'asiCreateInstance',
    'asiDestroyInstance',
    'asiGetLicenseeString',
    'asiGetVersion',
    'asiGetVersionString',
    'asiInitLibrary',
]
