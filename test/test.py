import ptst


@ptst.fixture(autouse=True, scope='session')
def sd_fixture_ptst():
    pass


def test_chan1():
    print("test_chan1")
    pass


def non_test():
    print("non_test")
    pass
