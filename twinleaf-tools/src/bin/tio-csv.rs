//Metadata to CSV convertor tool 
use std::fs::OpenOptions;
use std::io::*;
use std::env;
use twinleaf::data::{DeviceDataParser, ColumnData};
use twinleaf::tio;

fn read_in_csv(args: &[String], id: u32) -> std::io::Result<()> {
    let mut parser = DeviceDataParser::new(args.len() > 1);
    let s = &args[1].replace("csv", "");
    let path= format!("{}{}.csv", s, id).to_string();

    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    let mut streamhead: bool = false;
    let mut first: bool = true;

    for path in args {
        let mut rest: &[u8] = &std::fs::read(path).unwrap();
        while rest.len() > 0 {
            let (pkt, len) = tio::Packet::deserialize(rest).unwrap();
            rest = &rest[len..];
            for sample in parser.process_packet(&pkt) {
                //match stream id
                if sample.stream.stream_id == id as u8 {
                    for col in &sample.columns {
                        let time = format!("{:.6}   ", sample.timestamp_end());
                        let value = match col.value {
                            ColumnData::Int(x) => format!("{}", x),
                            ColumnData::UInt(x) => format!("{}", x),
                            ColumnData::Float(x) => format!("{:.5}", x),
                            ColumnData::Unknown => "?".to_string(),
                        };
                    
                        //write in column names
                        if !streamhead{
                            let timehead = format!("{},", "time");
                            let _= file.write_all(timehead.as_bytes()); 
                            
                            for col in &sample.columns {
                                let header = format!("{},", col.desc.name);
                                file.write_all(header.as_bytes())?;
                            }
                            file.write_all(b"\n")?;
                            streamhead = true;
                        }
                        
                        //write in data
                        let timefmt = format!("{},", time);
                        let formatted_value = format!("{},", value );
                        if first{
                            let _= file.write_all(timefmt.as_bytes());
                            first = false;
                        }
                        file.write_all(formatted_value.as_bytes())?;
                        
                    }
                    file.write_all(b"\n")?;
                    first = true;
                }
            }
        }
    }
    Ok(())
}

fn main()  {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1{
        eprintln!("Usage: tio-csv <stream id> <metadata> <csv>");
        std::process::exit(1);
    }
    let id  = args[0].parse().unwrap_or_else(|_| {
        eprintln!("Error Invalid stream ID");
        std::process::exit(1);
    }); 

    let _ =read_in_csv(&args[1..], id);
} 