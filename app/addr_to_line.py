import os

import pyperclip

addr_text = pyperclip.paste()
addr_text = addr_text.split('\n')
addr_text = [x.strip() for x in addr_text]
addr_text = " ".join(addr_text)
command = "/Users/rqg/Library/Android/sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-addr2line " \
       "-e ./build/intermediates/merged_native_libs/debug/out/lib/arm64-v8a/libsurface.so " + addr_text
os.system(command)
