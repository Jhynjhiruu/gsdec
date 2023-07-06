mod args;

use std::fs::{read, write};

use args::{OperationMode, Options};

const KEY: [u32; 0x10] = [
    0x1471332E, 0x8149432E, 0x75697B21, 0x15597883, 0x1C2AD435, 0x13ADE834, 0xE2DE18B1, 0x51BC7835,
    0x158732D4, 0x68D77612, 0x55424441, 0xD1F3FE22, 0xAEED7894, 0x34685312, 0xA3266563, 0x452CC12E,
];

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let filedata = read(options.infile)?;

    let decdata = filedata
        .chunks(4)
        .enumerate()
        .flat_map(|(index, e)| {
            match options.mode {
                OperationMode::Enc => {
                    u32::from_le_bytes(e.try_into().unwrap())
                        .wrapping_add(KEY[index % 0x10] & 0x0000FF00)
                        ^ KEY[index % 0x10]
                }
                OperationMode::Dec => (u32::from_le_bytes(e.try_into().unwrap())
                    ^ KEY[index % 0x10])
                    .wrapping_sub(KEY[index % 0x10] & 0x0000FF00),
            }
            .to_le_bytes()
        })
        .collect::<Vec<_>>();

    write(options.outfile, decdata).map_err(|e| e.into())
}
