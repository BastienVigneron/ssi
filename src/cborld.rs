use crate::error::Error;
use json::object::Object;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub fn get_contextmap() -> HashMap<i32, String> {
    let contextmap = HashMap::<i32, String>::from([
        (0x10, String::from("https://www.w3.org/ns/activitystreams")),
        (0x11, String::from("https://www.w3.org/2018/credentials/v1")),
        (0x12, String::from("https://www.w3.org/ns/did/v1")),
        (
            0x13,
            String::from("https://w3id.org/security/suites/ed25519-2018/v1"),
        ),
        (
            0x14,
            String::from("https://w3id.org/security/suites/ed25519-2020/v1"),
        ),
        (0x15, String::from("https://w3id.org/cit/v1")),
        (0x16, String::from("https://w3id.org/age/v1")),
        (
            0x17,
            String::from("https://w3id.org/security/suites/x25519-2020/v1"),
        ),
        (0x18, String::from("https://w3id.org/veres-one/v1")),
        (0x19, String::from("https://w3id.org/webkms/v1")),
        (0x1A, String::from("https://w3id.org/zcap/v1")),
        (
            0x1B,
            String::from("https://w3id.org/security/suites/hmac-2019/v1"),
        ),
        (
            0x1C,
            String::from("https://w3id.org/security/suites/aes-2019/v1"),
        ),
        (0x1D, String::from("https://w3id.org/vaccination/v1")),
        (
            0x1E,
            String::from("https://w3id.org/vc-revocation-list-2020/v1"),
        ),
        (0x1F, String::from("https://w3id.org/dcc/v1c")),
        (0x20, String::from("https://w3id.org/vc/status-list/v1")),
    ]);
    return contextmap;
}

pub fn get_keywordsmap() -> HashMap<String, i32> {
    let keywords = HashMap::<String, i32>::from([
        (String::from("@context"), 0),
        (String::from("@type"), 2),
        (String::from("@id"), 4),
        (String::from("@value"), 6),
        (String::from("@direction"), 8),
        (String::from("@graph"), 10),
        (String::from("@graph"), 12),
        (String::from("@index"), 14),
        (String::from("@json"), 16),
        (String::from("@language"), 18),
        (String::from("@list"), 20),
        (String::from("@nest"), 22),
        (String::from("@reverse"), 24),
        //digitalbazaar might remove the following
        (String::from("@base"), 26),
        (String::from("@container"), 28),
        (String::from("@default"), 30),
        (String::from("@embed"), 32),
        (String::from("@explicit"), 34),
        (String::from("@none"), 36),
        (String::from("@omitDefault"), 38),
        (String::from("@prefix"), 40),
        (String::from("@preserve"), 42),
        (String::from("@protected"), 44),
        (String::from("@requireAll"), 46),
        (String::from("@set"), 48),
        (String::from("@version"), 50),
        (String::from("@vocab"), 52),
    ]);
    return keywords;
}

pub struct EncodingOptions {
    jsonldDocument: serde_json::Value,
    documentloader: Object,
    contextMap: HashMap<i32, String>,
    keywordsMap: HashMap<String, i32>,
}

pub struct KeyWords {
    keywords: Vec<HashMap<String, i32>>,
}

#[derive(Deserialize)]
pub struct jsonldDoc {
    context: String,
    id: String,
    type_: String,
    verifiableCredential: Vec<Value>,
}

pub type ContextResult = Result<HashMap<i32, String>, Error>;

//(spec chapter 3.1)

pub async fn json_ld_to_cbor_ld(jsonldDocument: Value) -> Result<Vec<i32>, Error> {
    let mut result = vec![];

    //get context url algorithm (spec chapter 3.5)
    let context_urls: ContextResult = get_contexturl(jsonldDocument).await;
    if context_urls.is_err() {
        // generate uncompressed cbor (spec chapter 3.2)
        result.push(0x05);
        result.push(0x00);
        //let suffix = serde_cbor::to_writer(jsonldDocument);
        return Err(Error::UnsupportedProperty);
    } else {
        // generate compressed cbor (spec chapter 3.3)
        result.push(0xd9);
        result.push(0x05);
        result.push(1);
        //let suffix = encode();
    }

    //println!("result: {}", result);
    unimplemented!();
}

//(spec chapter 3.5)
pub async fn get_contexturl(jsonld_document: Value) -> Result<HashMap<i32, String>, Error> {
    let result = HashMap::<i32, String>::new();

    //let mut transformMaps = vec![];
    //handle one or multiple jsonldDocuments
    println!("doc {}", jsonld_document["@context"]);

    let keywords = get_keywordsmap();

    for (key, value) in jsonld_document.as_object().unwrap() {
        println!("{:?} ===> {:?}", key, value);
    }

    println!("keywords :{}", keywords["@context"]);
    // jsonldDocument.for_each(|(key, value)| {
    //     println!("{key}: {value}");
    //     println!("----------------------");
    // });
    // for obj in doc {
    //     let mut transformMap = vec![];
    //     transform(obj, transformMap).await;
    //     transforMaps.push(&transformMap);
    // }

    unimplemented!()
}

//spec chapter 3.3
pub fn encode(encodingOptions: EncodingOptions) {
    //encodingOptions.contextMap = get_contextmap();
    //encodingOptions.keywordsMap = get_keywordsmap();

    //call compressor with a documentloader and an appcontextmapp
    //compressor brings in the different encoders

    // compressor extends transformer _transform
    // transformer uses the keywords
}

pub async fn transform(obj: Object, transformMaps: Vec<String>) {
    //handle embedded contexts
    //handle typed contexts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_json_ld_transformation() {
        use serde_json::Value;

        let doc: Value = serde_json::from_str(r#"{
          "@context":[
             "https://www.w3.org/2018/credentials/v1"
          ],
          "id":"urn:uuid:3978344f-8596-4c3a-a978-8fcaba3903c5",
          "type":[
             "VerifiablePresentation"
          ],
          "verifiableCredential":[
             {
                "@context":[
                   "https://www.w3.org/2018/credentials/v1",
                   "https://w3id.org/age/v1",
                   "https://w3id.org/security/suites/ed25519-2020/v1"
                ],
                "id":"urn:uuid:188e8450-269e-11eb-b545-d3692cf35398",
                "type":[
                   "VerifiableCredential",
                   "OverAgeTokenCredential"
                ],
                "issuer":"did:key:z6MkkUbCFazdoducKf8SUye7cAxuicMdDBhXKWuTEuGA3jQF",
                "issuanceDate":"2022-06-13T14:02:44Z",
                "expirationDate":"2022-09-13T14:02:44Z",
                "credentialSubject":{
                   "overAge":21,
                   "concealedIdToken":"zo58FV8vqzY2ZqLT4fSaVhe7CsdBKsUikBMbKridqSyc7LceLmgWcNTeHm2gfvgjuNjrVif1G2A5EKx2eyNkSu5ZBc6gNnjF8ZkV3P8dPrX8o46SF"
                },
                "proof":{
                   "type":"Ed25519Signature2020",
                   "created":"2022-06-13T14:02:44Z",
                   "verificationMethod":"did:key:z6MkpH7YDw3LBmqTmUzifCBe999t8DatvWnpxSgYQn9UEeyc#z6MkpH7YDw3LBmqTmUzifCBe999t8DatvWnpxSgYQn9UEeyc",
                   "proofPurpose":"assertionMethod",
                   "proofValue":"zw9xkGemVTwufB3CeRfiBN6s5RE1Xzfdca76sNHc1G69bTcKu8SemBtGQcXGRqaicFBJcpKzzf9kNeCvhkSt1dsD"
                }
             }
          ]
       }
        "#).unwrap();

        let result = json_ld_to_cbor_ld(doc).await;
        panic!();
    }
}
