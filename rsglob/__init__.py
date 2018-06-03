"""Filename globbing utility."""

__all__ = ["glob", "iglob", "escape"]

VERSION = '0.0.1'

try:
    import rsglob._rsglob as _glob

    def glob(pathname, *, recursive=False):
        if isinstance(pathname, str):
            return _glob.glob_str(pathname)
        elif isinstance(pathname, bytes):
            # FIXME: better way ?
            return list(map(str.encode, _glob.glob_str(pathname.decode())))

    def iglob(pathname, *, recursive=False):
        # FIXME: better way ?
        return iter(glob(pathname, recursive=recursive))

    def escape(pathname):
        pass

except:
    print("no _rsglob exist")
