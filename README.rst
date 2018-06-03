========================================
rsglob - Python glob module in Rust
========================================


.. contents:: Table of Contents


Why ?
========================================

For fun :P

I want to try Rust's ecosystem (PyO3 and other stuff)
to see how hard is it to implement a module in Python standard library.



What has been done ?
========================================

The ``glob`` function can work right now,
but the ``iglob`` function is still WIP
(it isn't really return a iterator directly right now).

There are still some behavior difference for ``glob`` function,
that part is WIP too.



How can I use it ?
========================================

clone the source and built it with:

.. code-block:: sh

    python3 setup.py bdist_wheel



TODO
========================================

* iterator for iglob
* escape function
* check behavior for glob function
