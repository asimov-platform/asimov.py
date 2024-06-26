# This is free and unencumbered software released into the public domain.

"""Installation script for the 'asimov.py' package."""

# See: https://packaging.python.org/en/latest/discussions/setup-py-deprecated/
# See: https://packaging.python.org/en/latest/guides/modernize-setup-py-project/

from codecs import open
from os import path
from setuptools import find_packages, setup

def readfile(*filepath):
    with open(path.join(*filepath), encoding='utf-8') as f:
        return f.read()

PWD = path.abspath(path.dirname(__file__))

VERSION = readfile(PWD, 'VERSION').rstrip()
LONG_DESCRIPTION = readfile(PWD, 'README.md')

setup(
    name='asimov.py',
    version=VERSION,
    description="Asimov Software Development Kit (SDK) for Python",
    long_description=LONG_DESCRIPTION,
    long_description_content_type='text/markdown',
    url='https://sdk.asimov.so',
    author='Asimov AI',
    author_email='support@asimov.so',
    license='Public Domain',
    classifiers=[
        'Development Status :: 2 - Pre-Alpha',
        'Environment :: Console',
        'Intended Audience :: Developers',
        'Intended Audience :: Information Technology',
        'License :: Public Domain',
        'Natural Language :: English',
        'Operating System :: OS Independent',
        'Programming Language :: Python :: 3 :: Only',
        'Topic :: Software Development :: Libraries',
    ],
    keywords='',
    project_urls={
        'Bug Tracker': 'https://github.com/AsimovPlatform/asimov.py/issues',
        'Changelog': 'https://github.com/AsimovPlatform/asimov.py/blob/master/CHANGES.md',
        'Documentation': 'https://github.com/AsimovPlatform/asimov.py/blob/master/README.md',
        'Source Code': 'https://github.com/AsimovPlatform/asimov.py',
    },
    packages=find_packages(where='src'),
    package_dir={'': 'src'},
    install_requires=[
        'know.py>=0.0.2',
    ],
    python_requires='~=3.9',
    extras_require={'test': ['pytest']},
)
