use sgx_isa::{Report, Targetinfo};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


// read targetinfo of verifying enclave from stream,
// then create a report based on it and write it to the stream
fn handle_targetinfo(stream: &mut TcpStream) {

    // read from stream
    let mut buf = [0; Targetinfo::UNPADDED_SIZE];
    match stream.read_exact(&mut buf) {
    Ok(_) => {
        // copy data into targetinfo structure
        let targetinfo = Targetinfo::try_copy_from(&buf).unwrap();

        // create report for current enclave based on targetinfo
        let report = Report::for_target(&targetinfo, &[0; 64]); 
        let report: &[u8] = report.as_ref();
        stream.write(&report.to_vec()).unwrap();
        },
        Err(e) => {
            println!("Failed to receive data: {}", e); 
        }
    }
 }

 
fn main() {
    println!("\nListening on port 1035....\n");

    for stream in TcpListener::bind("localhost:1035").unwrap().incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                handle_targetinfo(&mut stream);
            },
            Err(_) => {
               println!("error occurred\n");
            }
        }
    }
}
