import sys

import wasmtime.loader


def app(args):
    print("Now do something!")

if __name__ == "__main__":
    app(sys.argv[1:])