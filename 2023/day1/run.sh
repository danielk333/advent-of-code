echo "ODIN"
echo "-----------"
cd odin-impl
odin build .
time ./odin-impl
cd ..

echo "RUST"
echo "-----------"
cd rust-impl
cargo build -r
time ./target/release/rust-impl
cd ..

echo "PYTHON"
echo "-----------"
cd python-impl
time python main.py
cd ..

echo "C"
echo "-----------"
cd c-impl
./build.sh
time ./main
cd ..

echo "GO"
echo "-----------"
cd go-impl
go build main.go
time ./main
cd ..
