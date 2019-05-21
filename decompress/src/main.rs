use std::fs::File;
use std::io::{Read, Write, Seek};
use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let mut f = File::open("cinco.res").unwrap();
    let mut data = vec!();
    let mut out_data = vec!();
    f.read_to_end(&mut data).unwrap();
    let fileLength = data.len();
    
    println!("{}", fileLength);
        

    let mut i = 0;
    let mut r = std::io::Cursor::new(&data);
    while (r.position() as usize) < fileLength {
        println!("cursor: {0}, len: {1}, i: {2}", r.position(), fileLength, i);
        
        r.read_u8().unwrap();
        let dec_size = r.read_u32::<LittleEndian>().unwrap();
        r.set_position(r.position()-5);
        // r.seek(std::io::SeekFrom::Current(-5)).unwrap();
        println!("{}", dec_size);
        // if dec_size as usize > fileLength - (r.position() as usize) {
        //     out_data.extend(&data[r.position() as usize..]);
        //     break;
        // }
        match quicklz::decompress(&mut r, data.len() as u32) {
            Ok(dec) => {
                println!("{}", dec.len());
                out_data.extend(&dec);
            },
            Err(quicklz::Error::SizeLimitExceeded {dec:_, max:_}) => {
                r.set_position(r.position()-9);
                out_data.extend(&data[r.position() as usize..]);
                break;
            },
            Err(e) => {
                panic!(e);
            }
        };

        i+=1;
    }

    let mut out = File::create("out.res").unwrap();
    out.write_all(&out_data).unwrap();
}
