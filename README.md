# rich-sdl2-rust

The [SDL2](https://github.com/libsdl-org/SDL/tree/SDL2) wrapper for Rust.

## First

This code is fork from [MikuroXina/rich-sdl2-rust](https://github.com/MikuroXina/rich-sdl2-rust)

Making personal changes.

## Concept

It won't be the binding, but the wrapper library for SDL2.

## Requestments

If Not use vendor (cargo feature), then You have to provide your own SDL library and header file.

- [SDL 2.0](https://github.com/libsdl-org/SDL/tree/SDL2)
- [SDL2_tiff 2.0](https://github.com/libsdl-org/SDL_ttf/tree/SDL2)
- [SDL2_mixer 2.0](https://github.com/libsdl-org/SDL_mixer/tree/SDL2)

Use generates Rust FFI bindings to C (and some C++) libraries ([bindgen in Rust](https://github.com/rust-lang/rust-bindgen)). bindgen is use [LLVM](https://llvm.org/).

## cargo feature

Default dynamic link library. Change static link library is `default = ["static"]`

Default not use SDL2 TTF and Mixer.  Use SDL2 TTF and Mixer is `default = ["ttf", "mixer"]`

Use vendor SDL2 library is `default = ["vendor"]`.

Exsample:
```
[dependencies]
rich-sdl2-rust = { default = ["static", "ttf", "mixer", "vendor"] }
```

## Document

Rust Docs: [rich-sdl2-rust](https://aquabindi.github.io/rich-sdl2-rust/rich_sdl2_rust/)

## Sister projects

- [MikuroXina/rich-sdl2-image-rust](https://github.com/MikuroXina/rich-sdl2-image-rust) - The wrapper of SDL_image 2.0.
- [MikuroXina/rich-sdl2-net-rust](https://github.com/MikuroXina/rich-sdl2-net-rust) - The wrapper of SDL_net 2.0.

## License

[Apache License, Version 2.0](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0
