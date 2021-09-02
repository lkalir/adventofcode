from setuptools import setup, find_packages
from setuptools_cythonize import get_cmdclass

setup(name='adventofcode',
      version='0.0.1',
      packages=find_packages(),
      author='Liam Kalir',
      author_email='liam.kalir@protonmail.com',
      entry_points={"console_scripts": {"aocpy = adventofcode.__main__:main"}},
      requires=['pytest', 'pytest_benchmark'],
      test_suite="tests",
      cmdclass=get_cmdclass())
