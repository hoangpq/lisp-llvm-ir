(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (import "env" "scanf" (func (;0;) (type 0)))
  (import "env" "printf" (func (;1;) (type 0)))
  (func (;2;) (type 1) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 1024
    i32.store offset=32
    local.get 0
    local.get 0
    i32.const 60
    i32.add
    i32.store offset=36
    local.get 0
    i32.const 32
    i32.add
    call 0
    drop
    local.get 0
    i32.const 1027
    i32.store offset=16
    local.get 0
    local.get 0
    i32.const 56
    i32.add
    i32.store offset=20
    local.get 0
    i32.const 16
    i32.add
    call 0
    drop
    local.get 0
    i32.const 20
    i32.store offset=52
    local.get 0
    i32.const 1030
    i32.store
    local.get 0
    local.get 0
    i32.load offset=60
    local.get 0
    i32.load offset=56
    i32.sub
    i32.const -20
    i32.add
    local.tee 1
    i32.store offset=48
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    call 1
    drop
    local.get 0
    i32.const 64
    i32.add
    global.set 0
    local.get 1)
  (func (;3;) (type 2) (param i32 i32) (result i32)
    call 2)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 2)
  (global (;0;) (mut i32) (i32.const 66592))
  (export "memory" (memory 0))
  (export "main" (func 3))
  (data (;0;) (i32.const 1024) "%u\00%u\00Result: %d\0a\00"))
