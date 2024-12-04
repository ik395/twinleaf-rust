use std::fs;
use std::io::Read;
use serde::ser::StdError;


#[derive(serde::Deserialize)]
struct Threshold {
    min: f32,
    max: f32,
}

impl Threshold {
    fn new( min: f32, max: f32) -> Self{
        Threshold {
            min,
            max,
        }
    }
    fn within_range(&self, value: f32) -> bool {
        value >= self.min && value <= self.max
    }

    fn no_value(&self) -> bool {
        self.min == self.max
    }
}

#[derive(serde::Deserialize)]
struct Thresholds {
    field: Option<Threshold>,
    status: Option<Threshold>,
    signal: Option<Threshold>,
    signal_detector: Option<Threshold>,
    laser_therm_control_error: Option<Threshold>,
    laser_therm_control_monitor: Option<Threshold>,
    laser_therm_heater_power: Option<Threshold>,
    cell_therm_sensor_measure: Option<Threshold>,
    cell_therm_heater_power: Option<Threshold>,
    vco_error: Option<Threshold>,
    vco_pull: Option<Threshold>,
    mcu_therm: Option<Threshold>
}

impl Thresholds{
    fn new() -> Self{
        Thresholds {
            field: Some(Threshold::new(0.0,0.0)),
            status: Some(Threshold::new(0.0,0.0)),
            signal: Some(Threshold::new(0.0,0.0)),
            signal_detector: Some(Threshold::new(0.0,0.0)),
            laser_therm_control_error: Some(Threshold::new(0.0,0.0)),
            laser_therm_control_monitor: Some(Threshold::new(0.0,0.0)),
            laser_therm_heater_power: Some(Threshold::new(0.0,0.0)),
            cell_therm_sensor_measure: Some(Threshold::new(0.0,0.0)),
            cell_therm_heater_power: Some(Threshold::new(0.0,0.0)),
            vco_error: Some(Threshold::new(0.0,0.0)),
            vco_pull: Some(Threshold::new(0.0,0.0)),
            mcu_therm: Some(Threshold::new(0.0,0.0))
        }
    }

    fn update_from_yaml(&mut self, file_path:&str) -> std::result::Result<Thresholds, Box<dyn StdError >> {        
        let mut file = fs::File::open(file_path)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        //Deserialize from yaml
        let thresh: Thresholds = serde_yaml::from_str(&file_content)?;
        let threshs: Thresholds = serde_yaml::from_str(&file_content)?;

        if let Some(field) = thresh.field {
            self.field = Some(field);
        }
        if let Some(status) = thresh.status {
            self.status = Some(status);
        }
        if let Some(signal) = thresh.signal {
            self.signal = Some(signal);
        }
        if let Some(signal_detector) = thresh.signal_detector {
            self.signal_detector = Some(signal_detector);
        }
        if let Some(laser_therm_control_error) = thresh.laser_therm_control_error {
            self.laser_therm_control_error = Some(laser_therm_control_error);
        }
        if let Some(laser_therm_control_monitor) = thresh.laser_therm_control_monitor {
            self.laser_therm_control_monitor = Some(laser_therm_control_monitor);
        }
        if let Some(laser_therm_heater_power) = thresh.laser_therm_heater_power {
            self.laser_therm_heater_power = Some(laser_therm_heater_power);
        }

        if let Some(cell_therm_heater_power) = thresh.cell_therm_heater_power {
            self.cell_therm_heater_power = Some(cell_therm_heater_power);
        }
        if let Some(cell_therm_sensor_measure) = thresh.cell_therm_sensor_measure {
            self.cell_therm_sensor_measure = Some(cell_therm_sensor_measure);
        }
        if let Some(vco_error) = thresh.vco_error {
            self.vco_error = Some(vco_error);
        }
        if let Some(vco_pull) = thresh.vco_pull {
            self.vco_pull = Some(vco_pull);
        }
        if let Some(mcu_therm) = thresh.mcu_therm {
            self.mcu_therm = Some(mcu_therm);
        }

        
        Ok(threshs)
    }
}

pub fn test_range(column: String, value: f32, file_path: Option<String>) -> u32 {
    let mut thresholds: Thresholds = Thresholds::new();

    //should never unwrap since we're setting default in the main file but just in case 
    let path = file_path.unwrap_or(String::from("default.yaml"));

    if let Err(_) = thresholds.update_from_yaml(&path){
        return 1;
    }

    let result = match column.as_str() {
        "field" => &thresholds.field,
        "status" => &thresholds.status,
        "signal" => &thresholds.signal,
        "signal.detector" => &thresholds.signal_detector,
        "laser.therm.control.error" => &thresholds.laser_therm_control_error,
        "laser.therm.control.monitor" => &thresholds.laser_therm_control_monitor,
        "laser.therm.heater.power" => &thresholds.laser_therm_heater_power,
        "cell.therm.sensor.measure" => &thresholds.cell_therm_sensor_measure,
        "cell.therm.heater.power" => &thresholds.cell_therm_heater_power,
        "vco.error" => &thresholds.vco_error,
        "vco.pull" => &thresholds.vco_pull,
        "mcu.therm" => &thresholds.mcu_therm,
        _ => return 1,
    };

    if let Some(threshold) = result {
        if threshold.no_value(){
            return 1; //white if no values defined

        } else if  threshold.within_range(value) {
            return 2; //green
        
        } else{
            return 3; //red   
        }
    }
    1
}