#!/bin/sh
# Automatically generated by UnrealBuildTool
# *DO NOT EDIT*

set -o errexit
"/media/data/unreal5/Engine/Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/bin/clang++" -fuse-ld=lld -rdynamic -shared -Wl,--gdb-index -Wl,-rpath='${ORIGIN}' -Wl,-rpath-link='${ORIGIN}' -Wl,-rpath='${ORIGIN}'/.. -Wl,-rpath='${ORIGIN}'/../../../Engine/Binaries/ThirdParty/Qualcomm/Linux -Wl,-rpath='${ORIGIN}'/../../../Engine/Binaries/ThirdParty/OpenVR/OpenVRv1_5_17/linux64 -Wl,-rpath='${ORIGIN}'/../../../Engine/Binaries/ThirdParty/PhysX3/Unix/x86_64-unknown-linux-gnu -Wl,-rpath='${ORIGIN}'/../../../Engine/Binaries/ThirdParty/Intel/Embree/Embree2140/Linux/x86_64-unknown-linux-gnu/lib -Wl,--disable-new-dtags -Wl,--as-needed -Wl,--hash-style=gnu -Wl,--build-id -target x86_64-unknown-linux-gnu --sysroot="../Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu" -B"../Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/usr/lib" -B"../Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/usr/lib64" -L"../Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/usr/lib" -L"../Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/usr/lib64" -o "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.so" -Wl,@"/home/amon/Projects/space-anchor/client/Plugins/Renet/Intermediate/Build/Linux/B4D820EA/UnrealEditor/Development/Renet/libUnrealEditor-Renet-0008.so.response" -Wl,--start-group -lpthread -ldl -l:RenetLibrary.so -lUnrealEditor-CoreUObject -lUnrealEditor-Engine -lUnrealEditor-Core -lUnrealEditor-Projects -Wl,--unresolved-symbols=ignore-in-shared-libs -Wl,--end-group -lrt -lm -nodefaultlibs -LThirdParty/Unix/LibCxx/lib/Unix/x86_64-unknown-linux-gnu/ ThirdParty/Unix/LibCxx/lib/Unix/x86_64-unknown-linux-gnu/libc++.a ThirdParty/Unix/LibCxx/lib/Unix/x86_64-unknown-linux-gnu/libc++abi.a -lm -lc -lpthread -lgcc_s -lgcc
"/media/data/unreal5/Engine/Binaries/Linux/dump_syms" -c -o "/home/amon/Projects/space-anchor/client/Intermediate/Build/Linux/B4D820EA/UnrealEditor/Development/libUnrealEditor-Renet-0008.so.psym" "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.so"
"/media/data/unreal5/Engine/Binaries/Linux/BreakpadSymbolEncoder" "/home/amon/Projects/space-anchor/client/Intermediate/Build/Linux/B4D820EA/UnrealEditor/Development/libUnrealEditor-Renet-0008.so.psym" "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.sym"
"/media/data/unreal5/Engine/Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/bin/llvm-objcopy" --strip-all "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.so" "/home/amon/Projects/space-anchor/client/Intermediate/Build/Linux/B4D820EA/UnrealEditor/Development/libUnrealEditor-Renet-0008.so_nodebug"
"/media/data/unreal5/Engine/Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/bin/llvm-objcopy" --only-keep-debug "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.so" "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.debug"
"/media/data/unreal5/Engine/Extras/ThirdPartyNotUE/SDKs/HostLinux/Linux_x64/v20_clang-13.0.1-centos7/x86_64-unknown-linux-gnu/bin/llvm-objcopy" --add-gnu-debuglink="/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.debug" "/home/amon/Projects/space-anchor/client/Intermediate/Build/Linux/B4D820EA/UnrealEditor/Development/libUnrealEditor-Renet-0008.so_nodebug" "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.so.temp"
mv "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.so.temp" "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.so"
chmod 644 "/home/amon/Projects/space-anchor/client/Plugins/Renet/Binaries/Linux/libUnrealEditor-Renet-0008.debug"

