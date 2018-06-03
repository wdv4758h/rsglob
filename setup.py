import os
import sys
from setuptools import find_packages, setup, Extension

try:    # for pip >= 10
    from pip._internal.req import parse_requirements
except ImportError:
    from pip.req import parse_requirements

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call(
        [sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension


def get_requirements(filename):
    # parse_requirements() returns generator of pip.req.InstallRequirement instance
    install_requires = parse_requirements(
        os.path.join(ROOT_DIR, filename),
        session=False,
    )

    # requirements is a list of requirement
    requirements = list(map(lambda x: str(x).split()[0], install_requires))
    return requirements


ROOT_DIR = os.path.dirname(os.path.realpath(__file__))


version = __import__('rsglob').VERSION
setup_requires = ['setuptools-rust>=0.6.0']
install_requires = get_requirements('requirements.txt')
test_requires = get_requirements('requirements-test.txt')
rust_extensions = [RustExtension('rsglob._rsglob', 'Cargo.toml')]


setup(
    name='rsglob',
    version=version,
    url='https://github.com/wdv4758h/rsglob',
    author='Chiu-Hsiang Hsu',
    author_email='wdv4758h@gmail.com',
    description=('Python glob in Rust'),
    long_description=open("README.rst").read(),
    download_url="https://github.com/wdv4758h/rsglob/archive/v{}.zip".format(
        version
    ),
    license='BSD',
    tests_require=test_requires,
    install_requires=install_requires,
    packages=find_packages(),
    rust_extensions=rust_extensions,
    zip_safe=False,
    classifiers=[
        'Development Status :: 2 - Pre-Alpha',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: BSD License',
        'Operating System :: OS Independent',
        'Programming Language :: Python',
        # 'Programming Language :: Python :: 2',
        # 'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3',
        # 'Programming Language :: Python :: 3.4',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
    ],
)
