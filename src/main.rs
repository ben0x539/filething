use std::collections::HashMap;
use std::ffi::OsString;
use std::env;
use std::io;
use std::fmt;

mod walk;

struct ByteSize(u64);

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut n = self.0;
        let mut scale = 0;
        while n >= 1024 && scale < 4 {
            n >>= 10;
            scale += 1;
        }
        try!(write!(f, "{:4}", n));
        if scale >= 1 {
            let truncated_digits = self.0 - (n << (10 * scale));
            let decimals = (truncated_digits * 1000) >> (10 * (scale));
            const UNITS: &'static [&'static str] = &["K", "M", "G", "T"];
            try!(write!(f, ".{:03} {}", decimals, UNITS[scale - 1]));
        }
        Ok(())
    }
}

fn main() { main_().unwrap(); }
fn main_() -> io::Result<()> {
    let mut by_ext = HashMap::<OsString, (u64, u64)>::new();
    let args = env::args();
    for arg in args.skip(1) {
        for result in try!(walk::walk_dir(arg)) {
            let (path, metadata) = try!(result);
            if !metadata.file_type().is_file() { continue; }

            let ext = path.extension().unwrap_or("none".as_ref());
            let inserted = if let Some(r) = by_ext.get_mut(ext) {
                (*r).0 += 1;
                (*r).1 += metadata.len();
                true
            } else {
                false
            };
            if !inserted {
                by_ext.insert(ext.into(), (1, metadata.len()));
            }
        }
    }
    let mut results = by_ext.into_iter().collect::<Vec<_>>();
    results.sort_by(|&(_, (_, a)), &(_, (_, b))| Ord::cmp(&b, &a));
    for (ext, size) in results {
        println!("{:20} {:6} {:20}", ext.to_string_lossy(), size.0, ByteSize(size.1));
    }

    Ok(())
}
