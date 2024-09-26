use ip_check::ip_lookup;
fn main(){
    let ip = "1.5.0.12";
    let file_path = "locationv4.csv";
    ip_lookup::look_up(ip, file_path);

    // let decimal = ip_to_decimal(ip);
    // println!("The decimal representation of {} is {}", ip, decimal);
    // println!("Hello, world!");
}
