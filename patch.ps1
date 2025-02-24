# Unfortunately, the Pluto author refused to change the hook ABIs to the C ABI (with a fair reason),
# which means we'll have to patch it ourselves. See: https://github.com/PlutoLang/Pluto/issues/1107
git apply --ignore-whitespace --whitespace=fix patches/hook_c_abi.patch
