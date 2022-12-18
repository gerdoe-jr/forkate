use std::{fs::File, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use acme_lib::create_p384_key;
use acme_lib::persist::FilePersist;
use acme_lib::{Directory, DirectoryUrl, Error};

pub fn load_rustls_config<S: Into<String>>(
    contact_email: S,
    primary_name: S,
    prod_env: bool,
) -> rustls::ServerConfig {
    request_cert(contact_email, primary_name, prod_env).unwrap();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut BufReader::new(File::open("./certificates/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("./certificates/key.pem").unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("couldn't locate PKCS 8 private keys");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

fn request_cert<S: Into<String>>(
    contact_email: S,
    primary_name: S,
    prod_env: bool,
) -> Result<(), Error> {
    let url = if !prod_env {
        DirectoryUrl::LetsEncryptStaging
    } else {
        DirectoryUrl::LetsEncrypt
    };

    let persist = FilePersist::new("./certificates");
    let dir = Directory::from_url(persist, url)?;
    let acc = dir.account(&contact_email.into())?;
    let mut ord_new = acc.new_order(&primary_name.into(), &[])?;

    let ord_csr = loop {
        // are we done?
        if let Some(ord_csr) = ord_new.confirm_validations() {
            break ord_csr;
        }

        let auths = ord_new.authorizations()?;
        let chall = auths[0].http_challenge();
        let token = chall.http_token();
        let path = format!("./.well-known/acme-challenge/{}", token);
        let proof = chall.http_proof();

        update_challenge(path, proof)?;
        chall.validate(5000)?;
        ord_new.refresh()?;
    };

    let pkey_pri = create_p384_key();

    let ord_cert = ord_csr.finalize_pkey(pkey_pri, 5000)?;

    let _cert = ord_cert.download_and_save_cert()?;

    Ok(())
}

fn update_challenge(path: String, contents: String) -> Result<(), Error> {
    use std::fs::*;

    if let Ok(_) = File::open(&path) {
        remove_file(&path)?;
    }

    write(&path, contents)?;

    Ok(())
}
