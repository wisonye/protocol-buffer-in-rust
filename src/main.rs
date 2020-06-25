use protobuf::Message;
use protobuf::RepeatedField;
use protocol::device_response::{DeviceResponse, GpsLocation, Lbs};
use std::fs::File;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    // let sample_verb = File::open("ababillarse.dat")?;
    // let mut reader = BufReader::new(sample_verb);
    //
    // let mut proto_buff = ::protobuf::CodedInputStream::from_buffered_reader(&mut reader);
    // let mut verb = protocol::verb::Verb::new();
    // let merged = verb.merge_from(&mut proto_buff);
    //
    // match merged {
    // Ok(_) => println!("{:?}", verb.take_description()),
    // Err(e) => println!("{:?}", e),
    // }

    let mut lbs = Lbs::new();
    lbs.set_mcc(460);
    lbs.set_mnc(0);
    lbs.set_cell_id(13543);
    lbs.set_lac(8452);

    let mut gps_location = GpsLocation::new();
    gps_location.set_status("Unseal,Precise Positioning,East Longitude,North Latitude".to_string());
    gps_location.set_is_precise(true);
    gps_location.set_lat("39.058355".to_string());
    gps_location.set_lon("117.726006".to_string());
    gps_location.set_altitude(1);
    gps_location.set_speed(0.0);
    gps_location.set_direction(228);
    gps_location.set_gps_time("2020-03-12T06:18:54.000Z".to_string());
    gps_location.set_recv_time("2020-06-21T10:15:40.365Z".to_string());
    gps_location.set_lbs(lbs);

    let mut device_response = DeviceResponse::new();
    device_response.set_protocol_type("808".to_string());
    device_response.set_response_type("Location Data Upload".to_string());
    device_response.set_response_type_bytes_str("0200".to_string());
    device_response.set_device_id("010019326051".to_string());
    device_response.set_msg_seq_no("0054".to_string());
    device_response.set_binary_protocol("7E0200003B010019326051005400000000600000020253FBB307045B360001000000E420031214185401040000000230011FE107046000210434E7E9013CE703000000E8033000025B7E".to_string());

    let mut extra_info_desc_list = RepeatedField::new();
    extra_info_desc_list.push("Odometer: 2 KM".to_string());
    extra_info_desc_list.push("Wireless Network Signal Strength: 31".to_string());
    extra_info_desc_list.push( "LBS Info: Country Code - CN, Network identification - 0, Station LAC - 8452, Station CELL - 13543".to_string());
    extra_info_desc_list.push("Device Power: 60%".to_string());
    extra_info_desc_list.push("24 Switch Status: Network Type - 2G,Motor Released".to_string());

    device_response.set_extra_info_desc_list(extra_info_desc_list);

    let bytes = device_response.write_to_bytes().unwrap();
    println!("bytes size: {:?}", bytes.len());
    println!("bytes: {:?}", bytes);

    Ok(())
}
