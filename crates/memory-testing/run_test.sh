# Move to the root of the repository
cd "$(dirname "$0")"
cd ../../

OUTPUT_DIR="./crates/memory-testing/output"

mkdir -p $OUTPUT_DIR
rm $OUTPUT_DIR/*

if [ "$1" = "no-docker" ]; then
    cargo build -p memory-testing
    sudo python3 ./crates/memory-testing/capture_dumps.py ./target/debug/memory-testing $OUTPUT_DIR
else
    docker build -f crates/memory-testing/Dockerfile -t bitwarden/memory-testing .
    docker run --rm -it -v $OUTPUT_DIR:/output bitwarden/memory-testing 
fi

python3 ./crates/memory-testing/analyze_dumps.py $OUTPUT_DIR
