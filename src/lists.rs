use reqwest;

use std::io::Write;
use std::fs;
use std::fs::File;
use std::error::Error;
use serde::Deserialize;

const DNS_CONSIST_DIR: &'static str = ".dns-consist";
const DNS_LIST_CSV_FILENAME: &'static str = "dnslist.csv";

#[derive(Debug, Deserialize, Clone)]
pub struct DNSServer {
    pub ip_address: String,
    pub name: Option<String>,
    pub as_number: Option<u64>,
    pub as_org: Option<String>,
    pub country_code: String,
    pub city: Option<String>,
    pub version: Option<String>
}

pub fn update_dns_lists(url: &str) {
    println!("Fetching new DNS servers list from {}", url);
    let dns_list_response = reqwest::blocking::get(url)
	.expect("Failed to make request to DNS server.")
	.text()
	.expect("Failed to read DNS server list response body.");

    println!("Parsing fetched list once to ensure update doesn't break installation.");
    let dns_servers = parse_dns_csv(&dns_list_response)
	.expect("Failed to parse DNS server list CSV response.");

    write_csv_out(&dns_list_response);
    
    println!("Done with update. {} servers found.", dns_servers.len());
}

pub fn get_server_list_path() -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(DNS_CONSIST_DIR);
    path.push(DNS_LIST_CSV_FILENAME);
    path.to_str().unwrap().to_string()
}

pub fn parse_dns_csv(csv_str: &str) -> Result<Vec<DNSServer>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());
    let mut servers = Vec::<DNSServer>::new();
    for result in rdr.deserialize() {
	let record: DNSServer = result?;
	servers.push(record);
    }
    Ok(servers)
}

fn write_csv_out(csv: &str) {
    let mut dns_list_path = dirs::home_dir()
	.expect("Could not get user home directory.");
    dns_list_path.push(DNS_CONSIST_DIR);

    fs::create_dir_all(dns_list_path.to_str().unwrap())
	.expect("Could not create directory in user home directory.");

    dns_list_path.push(DNS_LIST_CSV_FILENAME);
    println!("Writing file to {}", dns_list_path.to_str().unwrap());
    
    let mut csv_file = File::create(dns_list_path.to_str().unwrap())
	.expect("Could not create file in dns-consist directory.");
    csv_file.write_all(&csv.as_bytes())
	.expect("Failed to write dns server list file.");
}
