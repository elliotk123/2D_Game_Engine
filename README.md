# Project architecture roughly

engine_core
 ├─ creates modules
 │    ├─ systemin
 │    ├─ logic
 │    ├─ physics
 │    ├─ render_sync
 │    ├─ compositor
 │    └─ systemout
 │
 ├─ wires buses
 │    ├─ systemin  → logic
 │    ├─ logic     → physics
 │    ├─ logic     → render_sync
 │    ├─ physics   → render_sync
 │    ├─ render_sync → compositor
 │    └─ compositor → systemout
 │
 ├─ calls bootstrap (once / reload)
 │    └─ produces policy objects
 │         ├─ palette
 │         ├─ visual IDs
 │         ├─ constants
 │         └─ global rules
 │
 ├─ injects policy into modules
 │    ├─ compositor ← palette
 │    ├─ logic      ← IDs / constants
 │    └─ (optionally physics ← constants)
 │
 ├─ owns main loop
 │
 └─ runs runtime
      ├─ systemin.run()
      ├─ logic.run()
      ├─ physics.run()
      ├─ render_sync.run()
      ├─ compositor.run()
      └─ systemout.run()




# Instructions for ubuntu 

- install rust using rustup : https://rust-lang.org/tools/install/
- install build essenial : sudo apt install build-essential
- install Cmake : sudo apt install cmake
- install sdl dependencies : https://github.com/libsdl-org/SDL/blob/main/docs/README-linux.md#build-dependencies 

