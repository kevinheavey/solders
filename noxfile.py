# type: ignore
import nox


@nox.session
def python(session):
    session.install(
        "pytest",
        "pytest-asyncio",
        "maturin",
        "sphinx",
        "based58",
        "pybip39",
        "typing-extensions",
        "jsonalias",
        "myst-parser",
        "mnemonic",
    )
    session.install(".", "--no-build-isolation")
    session.run("make", "test", external=True)
