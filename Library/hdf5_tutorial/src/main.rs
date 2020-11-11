extern crate hdf5;
use hdf5::File;

fn main() -> hdf5::Result<()> {
    use self::Color::*;

    let _ = hdf5::silence_errors();

    {
        let file = File::open("pixels.h5", "w")?;
        let colors = file.new_dataset::<Color>().create("colors", 2)?;
        colors.write(&[RED, BLUE])?;
        let group = file.create_group("dir")?;
        let pixels = group.new_dataset::<Pixel>().create("pixels", (2, 2))?;
        pixels.write(&[
            [Pixel { xy: (1, 2), color: RED }, Pixel { xy: (3, 4), color: BLUE }],
            [Pixel { xy: (5, 6), color: GREEN }, Pixel { xy: (7, 8), color: RED }],
        ])?;
    }
    
    {
        // read
        let file = File::open("pixels.h5", "r")?;
        let colors = file.dataset("colors")?;
        assert_eq!(colors.read_1d::<Color>()?, &[RED, BLUE]);
        let pixels = file.dataset("dir/pixels")?;
        assert_eq!(
            pixels.read_raw::<Pixel>()?,
            vec![
                Pixel { xy: (1, 2), color: RED },
                Pixel { xy: (3, 4), color: BLUE },
                Pixel { xy: (5, 6), color: GREEN },
                Pixel { xy: (7, 8), color: RED },
            ]
        );
    }
    Ok(())
}

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Color {
    RED = 1,
    GREEN = 2,
    BLUE = 3,
}

#[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Pixel {
    xy: (i64, i64),
    color: Color,
}
