UI for a 1986 radioshack TV that now houses an ipad and a [custom knob input device](https://github.com/bcorey/tv-controls). The Ipad is configured to only run safari, which is directed at the address this WASM application is hosted at. The application is controlled by keyboard input mapped from TV dial rotation.

## Running the application
`cargo run` to test the bevy app in desktop mode.
`just serve` to test in the browser.
`just build` to compile the latest changes for deployment on push.
