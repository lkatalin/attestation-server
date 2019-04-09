use sgx_isa::{Report, Targetinfo};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};


// read info from target enclave stream
// returns an io::Result for Targetinfo type; mutable stream as input
fn read_targetinfo(stream: &mut TcpStream) -> io::Result<Targetinfo> {

    // creates buffer of 0s of same size as Targetinfo type
    let mut buf = [0; Targetinfo::UNPADDED_SIZE];

    // read from the stream into buf as many bytes are are in buf (check error)
    stream.read_exact(&mut buf)?;

    // validates data read... NOT true result is good; when map_or evalutes to false it's bad;
    // if value read into anon array [0] is not equal to zero
    if !stream.read(&mut [0]).ok().map_or(false, |n| n == 0) {
        return Err(io::ErrorKind::InvalidData.into())
    }

    // unwrap (get 'some' option) if data not invalid and convert to Targetinfo type
    Ok(Targetinfo::try_copy_from(&buf).unwrap())
}


// TCP server generates local attestations on request
fn main() -> io::Result<()>{
    
    println!("\nListening on port 1025....\n");
    
    // iterate through streams in incoming TCP traffic on port 1
    for stream in TcpListener::bind("localhost:1025")?.incoming() {

        // print success

        // if no error, assign data to stream variable
        let mut stream = stream?;

        // read targetinfo from stream
        let targetinfo = read_targetinfo(&mut stream)?;

        // issue local attestation report
        let report = Report::for_target(&targetinfo, &[0; 64]);

        // print what was in the received Targetinfo
        println!("{:?}\n", report);
        println!("{:?}", targetinfo);

        // write report to stream
        stream.write_all(report.as_ref())?;
    }

    Ok(())
}
