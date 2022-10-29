# type: ignore
import nox


@nox.session
def python(session):
    session.install(
        "pytest",
        "maturin",
        "sphinx",
        "based58",
        "pybip39",
        "typing-extensions",
        "jsonalias",
    )
    session.install(".", "--no-build-isolation")
    session.run("make", "test", external=True)
