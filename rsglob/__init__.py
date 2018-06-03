VERSION = '0.0.1'

try:
    import rsglob._rsglob as _glob

    def glob(pathname, *, recursive=False):
        if isinstance(pathname, str):
            return _glob.glob_str(pathname)
        elif isinstance(pathname, bytes):
            # FIXME: better way ?
            return list(map(str.encode, _glob.glob_str(pathname.decode())))

except:
    print("no _rsglob exist")
