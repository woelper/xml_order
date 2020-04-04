use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;
use quick_xml::Writer;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesEnd, BytesStart};
use quick_xml::events::attributes::Attribute;
use clap::{App, Arg};

fn main() {


    let matches = App::new("xmlorder")
        .arg(
            Arg::with_name("input")
                .help("XML file to sort")
                .required(true)
        )
        .get_matches();

    let input_file = Path::new(matches.value_of("input").unwrap_or("You need an XML file"));
    
    if let Some(ext) = input_file.extension() {
        if ext.to_string_lossy().to_lowercase() != "xml" {
            println!("This is not an xml file. Exiting. I am so sorry.");
            return
        }
    }

    if !input_file.is_file() {
        println!("This file does not exist. I have nothing left to do. Have nice day.");
        return
    }

    let mut reader = Reader::from_file(input_file).unwrap();
    reader.trim_text(true);
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            // A Bytes start event, such as <scale> ... </scale>
            Ok(Event::Start(ref e))  => {

                // make a new element
                let mut elem = BytesStart::owned(e.name(), e.name().len());

                // collect existing attributes
                let mut attrs = e.attributes().map(|a| a.unwrap()).collect::<Vec<Attribute>>();
                // sort properties
                attrs.sort_by(|a, b| a.key.cmp(&b.key));
                
                // assign the sorted attibutes
                elem.extend_attributes(attrs);

                // writes the event to the writer
                assert!(writer.write_event(Event::Start(elem)).is_ok());
            },

            // An empty element, such as <scale/> without separate closing tag
            Ok(Event::Empty(ref e)) => {

                // same as above, make elem
                let mut elem = BytesStart::owned(e.name(), e.name().len());

                // collect existing attributes
                let mut attrs = e.attributes().map(|a| a.unwrap()).collect::<Vec<Attribute>>();
                attrs.sort_by(|a, b| a.key.cmp(&b.key));
                elem.extend_attributes(attrs);

                // This time, write an empty element
                writer.write_event(Event::Empty(elem)).unwrap();

            },
            Ok(Event::End(ref e)) => {
                assert!(writer.write_event(Event::End(BytesEnd::borrowed(e.name()))).is_ok());
            },
            Ok(Event::Eof) => break,
            Ok(e) => {
                assert!(writer.write_event(&e).is_ok())
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();

    let mut file = File::create(input_file).unwrap();
    file.write_all(&result).unwrap();

}
