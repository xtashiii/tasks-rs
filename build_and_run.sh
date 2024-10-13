#!/bin/bash

echo "Building rust project"
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build Failed"
    exit 1
fi

echo "Building vue project"
cd frontend || { echo "Frontend folder not found!"; exit;  }
npm run build

if [ $? -ne 0 ]; then
    echo "Frontend build failed!"
    exit 1
fi

cd ..

echo "Running the application..."
./target/release/tasks

if [ $? -ne 0 ]; then
    echo "Failed to run rust application!"
    exit 1
fi