extern crate core;

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::jwk::AlgorithmParameters;
use jsonwebtoken::{
    decode, decode_header, encode, jwk, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

// RFC7519: JWT由三部分组成，这三部分使用点号 . 连接在一起：
// 1. Header：包含了关于令牌类型和使用的签名算法的元数据。它通常包含两个字段：alg（算法）和 typ（令牌类型），比如
//  {
//   "alg": "HS256", （HMAC SHA-256）
//   "typ": "JWT"
//  }
// 2. Payload：包含了有关身份验证主体（例如用户）及其它声明的信息。它是JWT的主要内容部分。负载中可以包含一些预定义的声明，也可以包含自定义的声明。例如：
//  {
//   "sub": "1234567890", （subject）
//   "name": "John Doe",
//   "admin": true
//  }
// 3. Signature（签名）：使用私钥对头部和负载进行签名生成的一段字符串。它用于验证令牌的真实性和完整性。例如：
//  HMACSHA256(
//   base64UrlEncode(header) + "." +
//   base64UrlEncode(payload),
//   secret
//  )
// jwt常见字段详解
//   jwt claims
//    iss（issuer）、sub（subject）、aud(audience)、exp(expiration time)、nbf(not before)、iat(issued At)、jti（JWT ID）
// Header
//   typ（type）、cty（content type）

pub fn auth(jwt: &str, token: &str, aud: &str) -> bool {
    let jwks: jwk::JwkSet = serde_json::from_str(jwt).unwrap();
    let header = decode_header(token).expect("decode header failed,");
    if let Some(kid) = header.kid {
        if let Some(j) = jwks.find(&kid) {
            match &j.algorithm {
                AlgorithmParameters::RSA(rsa) => {
                    // 采用RSA的公钥
                    let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                    let mut validation = Validation::new(
                        Algorithm::from_str(j.common.key_algorithm.unwrap().to_string().as_str())
                            .unwrap(),
                    );
                    validation.set_audience(&[aud]);
                    validation.validate_exp = false;
                    let decode_token = decode::<HashMap<String, serde_json::Value>>(
                        token,
                        &decoding_key,
                        &validation,
                    )
                    .unwrap();
                    println!("{:?}", decode_token);
                    true
                }
                _ => unreachable!("this should be a RSA"),
            }
        } else {
            eprintln!("no matching JWK found for the given kid");
            false
        }
    } else {
        eprintln!("Token doesn't have a kid header field");
        false
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Claims {
    sub: String,
    company: String,
    exp: u64,
}

pub fn hs512_ser_encode(claims: &Claims, key: &[u8]) -> String {
    let header = Header {
        kid: Some("singing_key".to_owned()),
        alg: Algorithm::HS512,
        ..Default::default()
    };
    let token = match encode(&header, claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!("encode error"),
    };
    token
}

pub fn hs512_deser_decode(token: String, key: &[u8]) -> (Header, Claims) {
    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("decode with invalid token."),
            _ => panic!("decode other err: {:?}", err),
        },
    };
    (token_data.header, token_data.claims)
}

#[cfg(test)]
mod jwt_tests {
    use super::*;

    #[test]
    fn auth_it_works() {
        let token: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjFaNTdkX2k3VEU2S1RZNTdwS3pEeSJ9.eyJpc3MiOiJodHRwczovL2Rldi1kdXp5YXlrNC5ldS5hdXRoMC5jb20vIiwic3ViIjoiNDNxbW44c281R3VFU0U1N0Fkb3BhN09jYTZXeVNidmRAY2xpZW50cyIsImF1ZCI6Imh0dHBzOi8vZGV2LWR1enlheWs0LmV1LmF1dGgwLmNvbS9hcGkvdjIvIiwiaWF0IjoxNjIzNTg1MzAxLCJleHAiOjE2MjM2NzE3MDEsImF6cCI6IjQzcW1uOHNvNUd1RVNFNTdBZG9wYTdPY2E2V3lTYnZkIiwic2NvcGUiOiJyZWFkOnVzZXJzIiwiZ3R5IjoiY2xpZW50LWNyZWRlbnRpYWxzIn0.0MpewU1GgvRqn4F8fK_-Eu70cUgWA5JJrdbJhkCPCxXP-8WwfI-qx1ZQg2a7nbjXICYAEl-Z6z4opgy-H5fn35wGP0wywDqZpqL35IPqx6d0wRvpPMjJM75zVXuIjk7cEhDr2kaf1LOY9auWUwGzPiDB_wM-R0uvUMeRPMfrHaVN73xhAuQWVjCRBHvNscYS5-i6qBQKDMsql87dwR72DgHzMlaC8NnaGREBC-xiSamesqhKPVyGzSkFSaF3ZKpGrSDapqmHkNW9RDBE3GQ9OHM33vzUdVKOjU1g9Leb9PDt0o1U4p3NQoGJPShQ6zgWSUEaqvUZTfkbpD_DoYDRxA";
        let jwks: &str = r#"
{"keys":[{"alg":"RS256","kty":"RSA","use":"sig","n":"2V31IZF-EY2GxXQPI5OaEE--sezizPamNZDW9AjBE2cCErfufM312nT2jUsCnfjsXnh6Z_b-ncOMr97zIZkq1ofU7avemv8nX7NpKmoPBpVrMPprOax2-e3wt-bSfFLIHyghjFLKpkT0LOL_Fimi7xY-J86R06WHojLo3yGzAgQCswZmD4CFf6NcBWDcb6l6kx5vk_AdzHIkVEZH4aikUL_fn3zq5qbE25oOg6pT7F7Pp4zdHOAEKnIRS8tvP8tvvVRkUCrjBxz_Kx6Ne1YOD-fkIMRk_MgIWeKZZzZOYx4VrC0vqYiM-PcKWbNdt1kNoTHOeL06XZeSE6WPZ3VB1Q","e":"AQAB","kid":"1Z57d_i7TE6KTY57pKzDy","x5t":"1gA-aTE9VglLXZnrqvzwWhHsFdk","x5c":["MIIDDTCCAfWgAwIBAgIJHwhLfcIbNvmkMA0GCSqGSIb3DQEBCwUAMCQxIjAgBgNVBAMTGWRldi1kdXp5YXlrNC5ldS5hdXRoMC5jb20wHhcNMjEwNjEzMDcxMTQ1WhcNMzUwMjIwMDcxMTQ1WjAkMSIwIAYDVQQDExlkZXYtZHV6eWF5azQuZXUuYXV0aDAuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA2V31IZF+EY2GxXQPI5OaEE++sezizPamNZDW9AjBE2cCErfufM312nT2jUsCnfjsXnh6Z/b+ncOMr97zIZkq1ofU7avemv8nX7NpKmoPBpVrMPprOax2+e3wt+bSfFLIHyghjFLKpkT0LOL/Fimi7xY+J86R06WHojLo3yGzAgQCswZmD4CFf6NcBWDcb6l6kx5vk/AdzHIkVEZH4aikUL/fn3zq5qbE25oOg6pT7F7Pp4zdHOAEKnIRS8tvP8tvvVRkUCrjBxz/Kx6Ne1YOD+fkIMRk/MgIWeKZZzZOYx4VrC0vqYiM+PcKWbNdt1kNoTHOeL06XZeSE6WPZ3VB1QIDAQABo0IwQDAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBRPX3shmtgajnR4ly5t9VYB66ufGDAOBgNVHQ8BAf8EBAMCAoQwDQYJKoZIhvcNAQELBQADggEBAHtKpX70WU4uXOMjbFKj0e9HMXyCrdcX6TuYiMFqqlOGWM4yghSM8Bd0HkKcirm4DUoC+1dDMzXMZ+tbntavPt1xG0eRFjeocP+kIYTMQEG2LDM5HQ+Z7bdcwlxnuYOZQfpgKAfYbQ8Cxu38sB6q82I+5NJ0w0VXuG7nUZ1RD+rkXaeMYHNoibAtKBoTWrCaFWGV0E55OM+H0ckcHKUUnNXJOyZ+zEOzPFY5iuYIUmn1LfR1P0SLgIMfiooNC5ZuR/wLdbtyKtor2vzz7niEiewz+aPvfuPnWe/vMtQrfS37/yEhCozFnbIps/+S2Ay78mNBDuOAA9fg5yrnOmjABCU="]},{"alg":"RS256","kty":"RSA","use":"sig","n":"0KDpAuJZyDwPg9CfKi0R3QwDROyH0rvd39lmAoqQNqtYPghDToxFMDLpul0QHttbofHPJMKrPfeEFEOvw7KJgelCHZmckVKaz0e4tfu_2Uvw2kFljCmJGfspUU3mXxLyEea9Ef9JqUru6L8f_0_JIDMT3dceqU5ZqbG8u6-HRgRQ5Jqc_fF29Xyw3gxNP_Q46nsp_0yE68UZE1iPy1om0mpu8mpsY1-Nbvm51C8i4_tFQHdUXbhF4cjAoR0gZFNkzr7FCrL4On0hKeLcvxIHD17SxaBsTuCBGd35g7TmXsA4hSimD9taRHA-SkXh558JG5dr-YV9x80qjeSAvTyjcQ","e":"AQAB","kid":"v2HFn4VqJB-U4vtQRJ3Ql","x5t":"AhUBZjtsFdx7C1PFtWAJ756bo5k","x5c":["MIIDDTCCAfWgAwIBAgIJSSFLkuG8uAM8MA0GCSqGSIb3DQEBCwUAMCQxIjAgBgNVBAMTGWRldi1kdXp5YXlrNC5ldS5hdXRoMC5jb20wHhcNMjEwNjEzMDcxMTQ2WhcNMzUwMjIwMDcxMTQ2WjAkMSIwIAYDVQQDExlkZXYtZHV6eWF5azQuZXUuYXV0aDAuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0KDpAuJZyDwPg9CfKi0R3QwDROyH0rvd39lmAoqQNqtYPghDToxFMDLpul0QHttbofHPJMKrPfeEFEOvw7KJgelCHZmckVKaz0e4tfu/2Uvw2kFljCmJGfspUU3mXxLyEea9Ef9JqUru6L8f/0/JIDMT3dceqU5ZqbG8u6+HRgRQ5Jqc/fF29Xyw3gxNP/Q46nsp/0yE68UZE1iPy1om0mpu8mpsY1+Nbvm51C8i4/tFQHdUXbhF4cjAoR0gZFNkzr7FCrL4On0hKeLcvxIHD17SxaBsTuCBGd35g7TmXsA4hSimD9taRHA+SkXh558JG5dr+YV9x80qjeSAvTyjcQIDAQABo0IwQDAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBSEkRwvkyYzzzY/jPd1n7/1VRQNdzAOBgNVHQ8BAf8EBAMCAoQwDQYJKoZIhvcNAQELBQADggEBAGtdl7QwzpaWZjbmd6UINAIlpuWIo2v4EJD9kGan/tUZTiUdBaJVwFHOkLRsbZHc5PmBB5IryjOcrqsmKvFdo6wUZA92qTuQVZrOTea07msOKSWE6yRUh1/VCXH2+vAiB9A4DFZ23WpZikBR+DmiD8NGwVgAwWw9jM6pe7ODY+qxFXGjQdTCHcDdbqG2160nKEHCBvjR1Sc/F0pzHPv8CBJCyGAPTCXX42sKZI92pPzdKSmNNijCuIEYLsjzKVxaUuwEqIshk3mYeu6im4VmXXFj+MlyMsusVWi2py7fGFadamzyiV/bxZe+4xzzrRG1Kow/WnVEizfTdEzFXO6YikE="]}]}
"#;
        let audience: &str = "https://dev-duzyayk4.eu.auth0.com/api/v2/";
        let result = auth(jwks, token, audience);
        assert_eq!(result, true);
    }

    #[test]
    fn hs512_works() {
        let key = b"secret";
        let my_claims = Claims {
            sub: "subject example".to_owned(),
            company: "JWTC".to_string(),
            exp: 10000000000,
        };
        let token = hs512_ser_encode(&my_claims, key);
        let (header, claim) = hs512_deser_decode(token, key);
        println!("header: {:?}", header);
        println!("claims: {:?}", claim);
        assert_eq!(claim, my_claims);
    }
}
