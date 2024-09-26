pub mod ip_lookup {

    use std::net::Ipv4Addr;
    use std::str::FromStr;
    use std::cmp::Ordering;
    use std::error::Error;
    use csv::Reader;


    #[derive(Debug, Clone)]
    pub struct IpRange {
        start: u32,
        end: u32,
        country: String,
        region: String,
        city: String,
    }

    fn read_ip_ranges(file_path: &str) -> Result<Vec<IpRange>, Box<dyn Error>> {
        let mut rdr = Reader::from_path(file_path)?;
        let mut ip_ranges = Vec::new();
        
        for result in rdr.records() {
            let record = result?;
            let start: u32 = record[0].parse()?;
            let end: u32 = record[1].parse()?;
            let country = record[2].to_string();
            let region = record[4].to_string();
            let city = record[5].to_string();
            
            ip_ranges.push(IpRange { start, end, country, region, city });
        }

        Ok(ip_ranges)
    }

    fn find_ip_range(ip: u32, ranges: &[IpRange]) -> Option<IpRange> {
        ranges.binary_search_by(|range| {
            if ip < range.start {
                Ordering::Greater // Search the left side
            } else if ip > range.end {
                Ordering::Less // Search the right side
            } else {
                Ordering::Equal // IP is within this range
            }
        }).ok().map(|index| ranges[index].clone())
    }

    fn ip_to_decimal(ip: &str) -> Result<u32,String> {
        let ip = Ipv4Addr::from_str(ip);
        if ip.is_err() {
            return Err("Invalid IP address".into());
        }
        let ip = ip.unwrap();
        let octets = ip.octets();
        let decimal = (octets[0] as u32) << 24 
            | (octets[1] as u32) << 16 
            | (octets[2] as u32) << 8 
            | octets[3] as u32;
        Ok(decimal)
    }

    pub fn look_up(ip: &str, file_path: &str) -> Option<IpRange> {
        let ip_decimal_to_use = match ip_to_decimal(ip) {
            Err(e) => {
                println!("Error: {}", e);
                return None;
            },
            Ok(ip_decimal) => {
                ip_decimal
            }
        };
         let ip_ranges_to_use = match read_ip_ranges(file_path) {
            Err(e) => {
                println!("Error: {}", e);
                return None;
            },
            Ok(ip_ranges) => {
                ip_ranges
            }
        };
        match find_ip_range(ip_decimal_to_use, &ip_ranges_to_use[..]) {
            Some(range) => {
                println!("IP is in range: {:?}", range);
                Some(range)
            },
            None => {
                println!("IP not found in any range");
                None
            }
        }
    }
}
pub use crate::ip_lookup::look_up;
