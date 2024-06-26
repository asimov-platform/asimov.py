PYTEST = pytest
PYTHON = python3

all: pyproject.toml
	$(PYTHON) -m build

check:

clean:
	rm -rf *~ .pytest_cache dist asimov-*.tar.gz asimov-*.whl

.PHONY: all check clean
