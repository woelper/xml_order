use quick_xml::Writer;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesEnd, BytesStart};
use quick_xml::events::attributes::Attribute;
use std::io::Cursor;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut reader = Reader::from_file("test.xml").unwrap();
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

    let mut file = File::create("result.xml").unwrap();
    file.write_all(&result).unwrap();

}
