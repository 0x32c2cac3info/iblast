import sys
from distutils.core import setup

# sys.exit(1)


# The below code will never execute, however GitHub is particularly
# picky about where it finds Python packaging metadata.
# See: https://github.com/github/feedback/discussions/6456
#
# To be removed once GitHub catches up.
reqs = [
    "anyio==4.0.0",
    "click==8.1.7",
    "dill==0.3.7",
    "exceptiongroup==1.1.3",
    "graphql-core==3.2.3",
    "h11==0.14.0",
    "idna==3.4",
    "inquirerpy==0.3.4",
    "libcst==1.0.1",
    "markdown-it-py==3.0.0",
    "mdurl==0.1.2",
    "multiprocess==0.70.14",
    "mypy-extensions==1.0.0",
    "nestd==0.3.1",
    "pfzy==0.3.4",
    "prompt-toolkit==3.0.39",
    "Pygments==2.16.1",
    "python-dateutil==2.8.2",
    "python-multipart==0.0.6",
    "PyYAML==6.0.1",
    "rich==13.5.2",
    "robyn==0.38.0",
    "six==1.16.0",
    "sniffio==1.3.0",
    "starlette==0.31.1",
    "strawberry-graphql==0.208.0",
    "typer==0.9.0",
    "typing-inspect==0.9.0",
    "typing_extensions==4.7.1",
    "uvicorn==0.23.2",
    "uvloop==0.17.0",
    "watchdog==2.2.1",
    "wcwidth==0.2.6",
]

setup(
    name='iblastpy',
    install_requires=reqs,
)
