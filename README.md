# rusty-fibonacci-gui

GUI version of my [Rusty Fibonacci CLI](https://github.com/egriff89/rusty-fibonacci) utility, powered by [Svelte](https://svelte.dev/) and [Tauri](https://tauri.app/). Still very much a work in progress, but still technically works.

## Todo
- [ ] Fix issues with generating numbers when `nth` is larger than 47. Instead of getting a string, the client gets an array of strings with multiple elements containing different numbers, none of which are the correct value.
	- [ ] Look further into the `num-bigint` crate and/or `tauri::command`.
- [ ] Successfully return a `anyhow::Result<BigUint>` instead of straight `BigUint`.
	- [ ] Create custom `FibError` type and correctly implement `serde` on it.
- [ ] Add some form of styling to make it actually look nice.