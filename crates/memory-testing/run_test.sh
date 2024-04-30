set -eo pipefail

# Move to the root of the repository
cd "$(dirname "$0")"
cd ../../

BASE_DIR="./crates/memory-testing"

mkdir -p $BASE_DIR/output
rm $BASE_DIR/output/* || true

cargo build -p memory-testing --release

if [ "$1" = "no-docker" ]; then
    # This specifically needs to run as root to be able to capture core dumps
    sudo ./target/debug/capture-dumps ./target/debug/memory-testing $BASE_DIR
else
    docker build -f crates/memory-testing/Dockerfile -t bitwarden/memory-testing .
    docker run --rm -it -v $BASE_DIR:/output bitwarden/memory-testing 
fi

./target/debug/analyze-dumps $BASE_DIR
