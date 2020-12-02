#!/bin/bash

mkdir cbuild

# Download the protobuf c++ source code
wget https://github.com/dlfcn-win32/dlfcn-win32/archive/v1.2.0.tar.gz
tar xvf v1.2.0.tar.gz -C ./cbuild --strip-components=1

if [[ "$TRAVIS_OS_NAME" == "windows" ]]; then
  # Build and install protobuf to cbuild/install
  mkdir cbuild/install
  mkdir cbuild/cmake/solution
  cd cbuild/cmake/solution
  cmake -G "Visual Studio 15 2017 Win64" -DCMAKE_INSTALL_PREFIX=../../install ..
  cmake --build . --config Release
  cmake --build . --config Release --target install
  cd ../../..
else
  echo "dlfcn.h exists. Nothing to do."  
fi
