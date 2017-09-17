
use super::super::super::clap::ArgMatches;

use std::io::{self,Read, Write, BufReader, BufWriter, Stdin, Stdout };
use std::fs::{File,OpenOptions};

/// Generic Reader
pub enum GenReader {
    FileR(BufReader<File>),
    StdIn(BufReader<Stdin>)
}
impl GenReader {
    pub fn new(arg: &ArgMatches) -> io::Result<GenReader> {
        if arg.is_present("STDIN") {
            Ok(GenReader::StdIn(BufReader::with_capacity(16384, io::stdin())))
        } else {
            match arg.value_of("INPUT") {
                Option::None => panic!("Nothing to read"),
                Option::Some(path) => {
                    Ok(GenReader::FileR(BufReader::with_capacity(16384, OpenOptions::new()
                        .read(true).write(false).create(false).open(path)?)))
                }
            }
        }
    }
}
impl Read for GenReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            &mut GenReader::FileR(ref mut reader) => reader.read(buf),
            &mut GenReader::StdIn(ref mut reader) => reader.read(buf),
        }
    }
}

/// Generic Writer
pub enum GenWriter {
    FileW(BufWriter<File>),
    StdOut(BufWriter<Stdout>)
}
impl GenWriter {
    pub fn new(arg: &ArgMatches) -> io::Result<GenWriter> {
        if arg.is_present("STDOUT") {
            Ok(GenWriter::StdOut(BufWriter::with_capacity(16384, io::stdout())))
        } else {
            match arg.value_of("OUTPUT") {
                Option::None => panic!("Nothing to write"),
                Option::Some(path) => {
                    Ok(GenWriter::FileW(BufWriter::with_capacity(16384, OpenOptions::new()
                        .read(false).write(true).create(true).open(path)?)))
                }
            }
        }
    }
}
impl Write for GenWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            &mut GenWriter::FileW(ref mut w) => w.write(buf),
            &mut GenWriter::StdOut(ref mut w) => w.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            &mut GenWriter::FileW(ref mut w) => w.flush(),
            &mut GenWriter::StdOut(ref mut w) => w.flush(),
        }
    }
}

