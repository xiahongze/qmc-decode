use std::{
    fs::File,
    io::{Error as IOError, ErrorKind, Read, Write},
    path::PathBuf,
};

use clap::{App, Arg};
use glob::glob;

const PRIV_KEYS: [u8; 128] = [
    0xc3, 0x4a, 0xd6, 0xca, 0x90, 0x67, 0xf7, 0x52, 0xd8, 0xa1, 0x66, 0x62, 0x9f, 0x5b, 0x09, 0x00,
    0xc3, 0x5e, 0x95, 0x23, 0x9f, 0x13, 0x11, 0x7e, 0xd8, 0x92, 0x3f, 0xbc, 0x90, 0xbb, 0x74, 0x0e,
    0xc3, 0x47, 0x74, 0x3d, 0x90, 0xaa, 0x3f, 0x51, 0xd8, 0xf4, 0x11, 0x84, 0x9f, 0xde, 0x95, 0x1d,
    0xc3, 0xc6, 0x09, 0xd5, 0x9f, 0xfa, 0x66, 0xf9, 0xd8, 0xf0, 0xf7, 0xa0, 0x90, 0xa1, 0xd6, 0xf3,
    0xc3, 0xf3, 0xd6, 0xa1, 0x90, 0xa0, 0xf7, 0xf0, 0xd8, 0xf9, 0x66, 0xfa, 0x9f, 0xd5, 0x09, 0xc6,
    0xc3, 0x1d, 0x95, 0xde, 0x9f, 0x84, 0x11, 0xf4, 0xd8, 0x51, 0x3f, 0xaa, 0x90, 0x3d, 0x74, 0x47,
    0xc3, 0x0e, 0x74, 0xbb, 0x90, 0xbc, 0x3f, 0x92, 0xd8, 0x7e, 0x11, 0x13, 0x9f, 0x23, 0x95, 0x5e,
    0xc3, 0x00, 0x09, 0x5b, 0x9f, 0x62, 0x66, 0xa1, 0xd8, 0x52, 0xf7, 0x67, 0x90, 0xca, 0xd6, 0x4a,
];

fn get_key(i: usize) -> u8 {
    PRIV_KEYS[(i % 0x7FFF) & 0x7f]
}

fn encode(p: &PathBuf, out_dir: &PathBuf, buf_size: usize) -> Result<(), IOError> {
    println!("converting {:?}", p);
    let ext = match p
        .extension()
        .ok_or_else(|| IOError::new(ErrorKind::Other, "no extension"))?
        .to_str()
        .unwrap()
    {
        "qmc0" => "mp3",
        "qmc3" => "mp3",
        "qmcogg" => "ogg",
        "qmcflac" => "flac",
        "flac" => "qmcflac",
        "mp3" => "qmc0",
        _ => return Err(IOError::new(ErrorKind::Other, "unsupported extension")),
    };
    let p_out = out_dir
        .join(
            p.file_stem()
                .ok_or_else(|| IOError::new(ErrorKind::Other, "no filename"))?,
        )
        .with_extension(ext);
    let mut fin = File::open(p)?;
    let mut fout = File::create(&p_out)?;
    let mut buf = vec![0; buf_size];
    let mut offset: usize = 0;
    while let Ok(n) = fin.read(&mut buf) {
        buf.iter_mut()
            .take(n)
            .enumerate()
            .for_each(|(i, b)| *b ^= get_key(i + offset));
        offset += n;
        fout.write_all(&buf[..n])?;
        if n == 0 {
            break;
        }
    }
    println!("successfully wrote to {:?}, {}", p_out, offset);
    Ok(())
}
fn main() {
    let matches = App::new("qmc-decode")
        .version("0.1.0")
        .author("Hongze Xia <hongzex@gmail.com>")
        .about(
            "Encoding/decoding QMC files
    if the extension ends with `qmc0` or `qmc3`, convert it to `mp3`
    if the extension ends with `qmcflac`, convert it to `flac`
    if the extension ends with `qmcogg`, convert it to `ogg`",
        )
        .arg(
            Arg::with_name("INPUT")
                .help("list of files or glob expressions to convert")
                .required(true)
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("OUTDIR")
                .default_value(".")
                .long("outdir")
                .short("o")
                .help("output directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("BUFFER_SIZE")
                .default_value("1048576")
                .long("buffer-size")
                .help("buffer size for reading files")
                .env("BUFFER_SIZE"),
        )
        .get_matches();
    let out_dir = matches.value_of("OUTDIR").map(PathBuf::from).unwrap();
    let buf_size: usize = matches
        .value_of("BUFFER_SIZE")
        .unwrap()
        .parse()
        .expect("BUFFER_SIER must be integer");

    matches
        .values_of("INPUT")
        .unwrap()
        .into_iter()
        .filter_map(|p| glob(p).ok())
        .flat_map(|p| p)
        .filter_map(|res| res.ok())
        .for_each(|pb| {
            if let Err(err) = encode(&pb, &out_dir, buf_size) {
                eprintln!("failed to convert {:?} with {:?}", pb, err);
            }
        });
}
