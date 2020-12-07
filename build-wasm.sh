# turning LLVM IR into object files
llc \
  -march=wasm32 \
  -filetype=obj \
  output.ll

wasm-objdump -x output.o

wasm-ld \
  --no-entry \
  --export=main \
  -o output.wasm \
  --strip-all \
  --allow-undefined \
  output.o

# wasm2wat
wasm2wat output.wasm -o output.wat
