pub struct CertAuthority{
    ca:rcgen::CertifiedKey,
}

impl CertAuthority{
    pub fn new()->Result<Self,rcgen::Error> {
        let key_pair = rcgen::KeyPair::generate()?;
        let mut params = rcgen::CertificateParams::default();
        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        let cert = params.self_signed(&key_pair)?;
        Ok(Self { ca: rcgen::CertifiedKey { cert, key_pair } })
    }
    pub fn sign_for_host(&self,hostname:&str)->Result<(Vec<u8>,Vec<u8>),rcgen::Error>{
        let key_pair = rcgen::KeyPair::generate()?;
        let params = rcgen::CertificateParams::new(vec![hostname.to_string()])?;
        let cert =params.signed_by(&key_pair,&self.ca.cert,&self.ca.key_pair)?;
        Ok((cert.der().to_vec(), key_pair.serialize_der()))
    }
}

