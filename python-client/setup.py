from setuptools import setup
import os

def readme():
    with open(os.path.join(os.path.dirname(__file__), 'README.md'), encoding='utf-8') as f:
        return f.read()

setup(
    name='lockserver-client',
    version='0.1.1',
    description='Python client SDK for lockserver (distributed lock server)',
    long_description=readme(),
    long_description_content_type='text/markdown',
    author='Ben Liao',
    author_email='abenliao@gmail.com',
    py_modules=['lockserver_client'],
    install_requires=['requests'],
    url='https://github.com/benliao/lockserver',
    keywords='distributed lock server client sdk python',
    license='MIT',
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: OS Independent',
    ],
)
