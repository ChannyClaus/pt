def fixture(autouse=False, scope="function"):
    def pass_through(f):
        return f

    return pass_through
