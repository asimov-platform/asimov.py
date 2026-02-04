# This is free and unencumbered software released into the public domain.

__version__: str
__version_tuple__: tuple[int, int, int, str | None]

class ConfigDirectory:
    @staticmethod
    def home() -> ConfigDirectory:
        pass
    def __init__(self, path: str) -> None:
        pass
    def __repr__(self) -> str:
        pass
    def default_profile(self) -> ConfigProfile:
        pass

class ConfigProfile:
    name: str
    def __repr__(self) -> str:
        pass

class ModuleDirectory:
    @staticmethod
    def home() -> ModuleDirectory:
        pass
    def __init__(self, path: str) -> None:
        pass
    def __repr__(self) -> str:
        pass
    def is_installed(self, module_name: str) -> bool:
        pass
    def is_enabled(self, module_name: str) -> bool:
        pass

class ModuleNameIterator:
    pass

class ProgramDirectory:
    @staticmethod
    def home() -> ProgramDirectory:
        pass
    def __init__(self, path: str) -> None:
        pass
    def __repr__(self) -> str:
        pass

class StateDirectory:
    @staticmethod
    def home() -> StateDirectory:
        pass
    def __init__(self, path: str) -> None:
        pass
    def __repr__(self) -> str:
        pass
    def has_configs(self) -> bool:
        pass
    def has_modules(self) -> bool:
        pass
    def has_programs(self) -> bool:
        pass
    def configs(self) -> ConfigDirectory:
        pass
    def modules(self) -> ModuleDirectory:
        pass
    def programs(self) -> ProgramDirectory:
        pass
