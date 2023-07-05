mod args;

use std::fs::{read, write};

use args::{OperationMode, Options};

use anyhow::Result;

const KEY: [u32; 0x10] = [
    0x2E337114, 0x2E434981, 0x217B6975, 0x83785915, 0x35D42A1C, 0x34E8AD13, 0xB118DEE2, 0x3578BC51,
    0xD4328715, 0x1276D768, 0x41444255, 0x22FEF3D1, 0x9478EDAE, 0x12536834, 0x636526A3, 0x2EC12C45,
];

fn main() -> Result<()> {
    let options = Options::parse();

    let filedata = read(options.infile)?;

    let decdata = filedata
        .chunks(4)
        .enumerate()
        .flat_map(|(index, e)| {
            match options.mode {
                OperationMode::Enc => {
                    u32::from_be_bytes(e.try_into().unwrap())
                        .wrapping_add(KEY[index % 0x10] & 0x0000FF00)
                        ^ KEY[index % 0x10]
                }
                OperationMode::Dec => (u32::from_be_bytes(e.try_into().unwrap())
                    ^ KEY[index % 0x10])
                    .wrapping_sub(KEY[index % 0x10] & 0x0000FF00),
            }
            .to_be_bytes()
        })
        .collect::<Vec<_>>();

    write(options.outfile, decdata).map_err(|e| e.into())
}
