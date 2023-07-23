Fork that downgrades to a previous version (pre sys-tray, cannot bother to fix that right now) with some fixes glued together for use with Linux

# OsuNowPlaying

A program to display what you are currently playing on your osu! profile or elsewhere.

## Installation

- Head to [osu.bitknox.me](https://osu.bitknox.me) and sign in
- Click the download button to download all the required programs/config files
- Extract the zip file and run the program

## Usage

When the program is running the link to your image will be located at: <https://osu.bitknox.me/playing?uId=${YOUR_ID}>
You can add multiple parameters to the URL.

| Argument | Value | Description |
| --- | --- | --- |
| uId | id | Your profile ID (required) |
| width | number | Set the width of the image in pixels |
| type | svg/png | Set the image file type |

Example: <https://osu.bitknox.me/playing?uId=6971313&width=500&type=png>

The ID can be found by heading to your osu! profile and copying from the URL.

A redirect URL to the current map is also available <https://osu.bitknox.me/redirect?uId=${YOUR_ID}>
### Example(updates live)

![This is an image](https://osu.bitknox.me/playing?uId=6971313)

## Building from source

### Prerequisites

- Working installation of rust/cargo

### Building

```bash
git clone https://github.com/bitknox/OsuNP.git
cd OsuNP
cargo build --realease 
```

To build it yourself, you will need to download [gosumemory](https://github.com/l3lackShark/gosumemory) and edit the code to point to the desired exe location.

Further, you need to create a token.txt and place it beside the executable or edit the source. (token can be found by heading to [osu.bitknox.me](https://osu.bitknox.me))

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## Acknowledgements

[gosumemory](https://github.com/l3lackShark/gosumemory) for making this project possible

[TeQnix](https://github.com/TeQnix) for helping with design changes

## License

[GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html)
