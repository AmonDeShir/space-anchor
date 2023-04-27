#/bin/bash

rm -rf ./target
rm -rf ./x64
rm -rf ../../../Binaries/ThirdParty/RenetLibrary

cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-unknown-linux-gnu

mkdir -p ./x64/Release

cp ./target/x86_64-pc-windows-gnu/release/renet_library.dll ./x64/Release/RenetLibrary.dll
cp ./target/x86_64-unknown-linux-gnu/release/librenet_library.so ./x64/Release/RenetLibrary.so

gendef ./target/x86_64-pc-windows-gnu/release/renet_library.dll
x86_64-w64-mingw32-dlltool -d ./renet_library.def -l ./x64/Release/RenetLibrary.lib -D ./target/x86_64-pc-windows-gnu/release/renet_library.dll
rm ./renet_library.def

mkdir -p ../../../Binaries/ThirdParty/RenetLibrary/Linux/x86_64-unknown-linux-gnu
mkdir -p ../../../Binaries/ThirdParty/RenetLibrary/Win64

cp ./x64/Release/RenetLibrary.dll ../../../Binaries/ThirdParty/RenetLibrary/Win64/RenetLibrary.dll
cp ./x64/Release/RenetLibrary.so ../../../Binaries/ThirdParty/RenetLibrary/Linux/x86_64-unknown-linux-gnu/RenetLibrary.so


