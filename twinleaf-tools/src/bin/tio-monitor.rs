use twinleaf::tio;
use twinleaf::data::{ColumnData, Device};
use tio::proto::DeviceRoute;
use tio::proxy;
use tio::util;

use getopts::Options;
use std::env;

use bytemuck::cast_slice;
use pancurses::*;


fn tio_opts() -> Options {
    let mut opts = Options::new();
    opts.optopt(
        "r",
        "",
        &format!("sensor root (default {})", util::default_proxy_url()),
        "address",
    );
    opts.optopt(
        "s",
        "",
        "sensor path in the sensor tree (default /)",
        "path",
    );
    opts
}

fn tio_parseopts(opts: Options, args: &[String]) -> (getopts::Matches, String, DeviceRoute) {
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    let root = if let Some(url) = matches.opt_str("r") {
        url
    } else {
        "tcp://localhost".to_string()
    };
    let route = if let Some(path) = matches.opt_str("s") {
        DeviceRoute::from_str(&path).unwrap()
    } else {
        DeviceRoute::root()
    };
    (matches, root, route)
}

fn stream(args: &[String]) {
    let opts = tio_opts();
    let (_matches, root, route) = tio_parseopts(opts, args);
    
    let proxy = proxy::Interface::new(&root);
    let device = proxy.device_rpc(route).unwrap();

    let column: String = device.get("data.stream.columns").unwrap(); 
    let mut names: Vec<String> = Vec::new();
   
    for name in column.split_whitespace() { 
        names.push(name.to_string());
    }

    //initialize terminal window
    let window = initscr();
    window.refresh();
    noecho();

    for pkt in proxy.tree_full().unwrap().iter() {
        if let tio::proto::Payload::LegacyStreamData(ref data) = pkt.payload {
            window.clear();
            let floats: &[f32] = cast_slice(&data.data);
            
            for (name, &value) in names.iter().zip(floats.iter()) { 
                println!("\n"); 
                window.refresh();                
                let string = format!("{}: {:?}", name.as_str(), value);               
                window.mvprintw(0,0, &string); 
            }   
        }
        
    }
    endwin();
}

fn run_monitor(args: &[String], path: &str) {
    let opts = tio_opts();
    let (_matches, root, route) = tio_parseopts(opts, args);

    let proxy = proxy::Interface::new(&root);
    let device = proxy.device_full(route).unwrap();
    let mut device = Device::new(device);

    //initialize terminal window
    let window = initscr();
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_RED, COLOR_BLACK);
    
    window.refresh();
    noecho();
    let mut y_position = 3;
    
    loop{
        let sample = device.next();

        let name = format!("Device Name: {}  Serial: {}   Session ID: {}", sample.device.name, sample.device.serial_number, sample.device.session_id);
        window.mvprintw(1,0, &name);

        for col in &sample.columns{
            let color_pair = twinleaf::monitor::range::test_range(col.desc.name.clone(), 
                match col.value {
                ColumnData::Int(x) => x as f32,
                ColumnData::UInt(x) => x as f32,
                ColumnData::Float(x) => x as f32,
                ColumnData::Unknown => 0.0,
                }, Some(path.to_string()));

            let string = format!(
                " {}: {}",
                col.desc.name,
                match col.value {
                    ColumnData::Int(x) => format!("{}", x),
                    ColumnData::UInt(x) => format!("{:.3}", x),
                    ColumnData::Float(x) => format!("{:.3}", x),
                    ColumnData::Unknown => "?".to_string(),
                }
            ); 
    
            if sample.stream.stream_id == 0x02 {
                y_position += 1;
            }

            window.attron(COLOR_PAIR(color_pair));
            window.mvprintw(y_position, 0, &string);                
            window.attroff(COLOR_PAIR(color_pair));
            window.refresh();
        }
        y_position = 3;
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();

    let default_path = "default.yaml".to_string();
    let args2 = args.get(2).unwrap_or(&default_path);
    
    match args[1].as_str() {
        "stream" => {
            stream(&args[1..])
        }
        "usb" => {
            run_monitor(&args[1..], args2);
        }
        _ => {
            println!("Usage:");
            println!("Note: running on bad/no yaml defaults to colorless values");
            println!("tio-monitor stream");
            println!("tio-monitor usb [yaml file_path]")
        }
    }

}

