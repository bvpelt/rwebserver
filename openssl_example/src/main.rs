use chrono::{DateTime, Utc};
use log::{debug, error, info};
use openssl::asn1::{Asn1Time, Asn1TimeRef};
use openssl::bn::{BigNum, MsbOption};
use openssl::error::ErrorStack;

use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::{X509, X509Name};
use openssl_example::AppConfig;
use std::fs::File;
use std::io::Write;

fn asn1_time_to_iso8601(asn1_time: &Asn1TimeRef) -> Result<String, Box<dyn std::error::Error>> {
    // Epoch anchor: 1970-01-01 00:00:00 UTC
    let epoch = Asn1Time::from_unix(0)?;

    // Find the difference in days and seconds relative to UNIX epoch
    let diff = epoch.diff(asn1_time)?;
    let total_seconds = (diff.days as i64 * 86400) + diff.secs as i64;

    let dt = DateTime::<Utc>::from_timestamp(total_seconds, 0).ok_or("Invalid timestamp")?;

    Ok(dt.to_rfc3339())
}

fn save_pem_files(
    cert: &X509,
    key: &PKey<Private>,
    cert_path: &str,
    key_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Export Certificate to PEM format and write to file
    let cert_pem = cert.to_pem()?;
    let mut cert_file = File::create(cert_path)?;
    cert_file.write_all(&cert_pem)?;

    // 2. Export Private Key to PKCS#8 / PEM format and write to file
    let key_pem = key.private_key_to_pem_pkcs8()?;
    let mut key_file = File::create(key_path)?;
    key_file.write_all(&key_pem)?;

    Ok(())
}

fn create_cert() -> Result<(X509, PKey<Private>), ErrorStack> {
    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?; // X509_VERSION 3

    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(160, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    // Convert serial_number Asn1Integer to BigNum
    if let Ok(bn) = serial_number.to_bn() {
        debug!("serial_number (hex): {:X}", bn);
        debug!("serial_number (dec): {}", bn);
    }
    cert_builder.set_serial_number(&serial_number)?;

    let mut name_builder = X509Name::builder()?;
    name_builder.append_entry_by_text("C", "UK")?;
    name_builder.append_entry_by_text("CN", "Our common name")?;
    let cert_name = name_builder.build();
    let name_str = cert_name
        .entries()
        .map(|e| {
            format!(
                "/{}={}",
                e.object().nid().short_name().unwrap_or("?"),
                e.data().to_string().unwrap_or_default()
            )
        })
        .collect::<String>();

    debug!("cert name issuer and subject: {}", name_str);

    cert_builder.set_issuer_name(&cert_name)?;
    cert_builder.set_subject_name(&cert_name)?;

    let not_before = Asn1Time::days_from_now(0)?;
    cert_builder.set_not_before(&not_before)?;

    let not_after = Asn1Time::days_from_now(365)?;
    cert_builder.set_not_after(&not_after)?;

    // Convert and log both timestamps
    if let Ok(nb_iso) = asn1_time_to_iso8601(&not_before) {
        debug!("not_before: {}", nb_iso);
    } else {
        error!("Error converting not_before to String");
    }

    if let Ok(na_iso) = asn1_time_to_iso8601(&not_after) {
        debug!("not_after:  {}", na_iso);
    } else {
        error!("Error converting not_after to String");
    }

    let private_key = PKey::from_rsa(Rsa::generate(3072)?)?;
    cert_builder.set_pubkey(&private_key)?;

    if let Ok(rsa) = private_key.rsa() {
        debug!("Private Key Details (RSA 3072):");
        debug!("  Public Exponent (e): {}", rsa.e());
        debug!("  Modulus bit length:  {}", rsa.size() * 8);
        if let Ok(modulus_hex) = rsa.n().to_hex_str() {
            debug!("  Modulus (n, hex):    {}", modulus_hex);
        }
    }

    cert_builder.sign(&private_key, MessageDigest::sha512())?;
    let cert = cert_builder.build();

    if let Ok(cert_text) = cert.to_text() {
        let text = String::from_utf8_lossy(&cert_text);
        debug!("Certificate Details:\n{}", text);
    }

    Ok((cert, private_key))
}

fn main() {
    pretty_env_logger::init_timed();
    dotenvy::dotenv().ok();

    let appconfig = AppConfig::from_env();
    info!("Launching openssl_example version: {}", appconfig.version);

    match create_cert() {
        Ok((cert, key)) => {
            info!("Successfully generated self-signed certificate!");
            if let Some(entry) = cert.subject_name().entries().next() {
                if let Ok(subject) = entry.data().to_string() {
                    info!("Certificate Subject CN: {}", subject);
                }
            }

            // Save PEM files to disk
            let cert_filename = "cert.pem";
            let key_filename = "key.pem";

            match save_pem_files(&cert, &key, cert_filename, key_filename) {
                Ok(_) => {
                    info!("Saved certificate to '{}'", cert_filename);
                    info!("Saved private key to '{}'", key_filename);
                }
                Err(e) => {
                    error!("Failed to save PEM files: {}", e);
                }
            }
        }
        Err(e) => {
            log::error!("Failed to create certificate: {}", e);
        }
    }
}
