
mod formatter_cli;
mod reader;
mod writer;
mod fsutil;

use self::writer::build_emitter_config;
use self::reader::build_parser_config;
use self::fsutil::{GenReader,GenWriter};

use std::io::{self,Write};

use super::super::clap::ArgMatches;


pub fn do_reformat(arg: &ArgMatches) -> io::Result<()> {
    let emitter = build_emitter_config(arg); 
    let writer = GenWriter::new(arg)?;
    let mut writer = emitter.create_writer(writer);

    let parser = build_parser_config(arg);
    let reader = GenReader::new(arg)?;
    let reader = parser.create_reader(reader);

    for event in reader {
        match event {
            Ok(var) => match var.as_writer_event() {
                Option::None => continue,
                Option::Some(var) => {
                    match writer.write(var) {
                        Ok(()) => continue,
                        Err(e) => panic!("XML Writter error occured {:?}", e)
                    }
                }
            },
            Err(e) => panic!("XML Reader error occured {:?}", e)
        };
    }
    let mut writer = writer.into_inner();
    writer.flush()?;
    Ok(())
}
