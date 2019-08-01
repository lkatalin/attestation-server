use sgx_isa::{Report, Targetinfo};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use ring::{agreement, rand};


// read QE ID from stream
fn handle_qe_id(stream: &mut TcpStream) -> io::Result<Targetinfo> {

    // read from stream - we don't know how long this is?
    let mut buf = [0; Targetinfo::UNPADDED_SIZE];
    match stream.read_exact(&mut buf) {
    Ok(_) => {
        let buf = Targetinfo::try_copy_from(&buf).unwrap();
        Ok(buf)

        // copy data into targetinfo structure
        //let targetinfo = Targetinfo::try_copy_from(&buf).unwrap();

        // create report for current enclave based on targetinfo
        //let report = Report::for_target(&targetinfo, &[0; 64]); 
        //let report: &[u8] = report.as_ref();
        //stream.write(&report.to_vec()).unwrap();
        },
        Err(e) => {
            println!("Failed to receive data: {}", e); 
            return Err(io::ErrorKind::InvalidData.into())
        }
    }
 }

fn create_report(t: Targetinfo) -> Report {
    Report::for_target(&t, &[0; 64])
}

fn create_report_with_data(t: Targetinfo, rd: [u8; 64]) -> Report {
    Report::for_target(&t, &rd)
}

fn create_session_key()  {// {-> ring::agreement::PublicKey {
    let rng = rand::SystemRandom::new();
    let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng);
    
    //let priv_key = agreement::EphemeralPrivateKey::generate(&agreement::ECDH_P256, &rng).expect("Could not generate private ECDH key.");
    //let pub_key = priv_key.compute_public_key().expect("Could not generate public ECDH key.");

    //pub_key
}

fn main() {
    println!("\nListening on port 1032....\n");

    for stream in TcpListener::bind("localhost:1032").unwrap().incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let qe_id = handle_qe_id(&mut stream).unwrap();

                //println!("QE ID received by enclave is: {:?}", qe_id);

                //let ecdh_pub_key = create_session_key();
                create_session_key();

                let mut reportdata = [0; 64];
                //reportdata.copy_from_slice(ecdh_pub_key.as_ref());

                let report = create_report_with_data(qe_id, reportdata);

                println!("Report Data is: {:?}", report.reportdata.to_vec());

                stream.write(&report.as_ref()).unwrap();
            },
            Err(_) => {
               println!("error occurred\n");
            }
        }
    }
}
