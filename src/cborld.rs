use std::collections::HashMap;

pub fn get_contextmap() -> HashMap {
    contextmap = HashMap::<Bytes, String>::from([
        (0x10, "https://www.w3.org/ns/activitystreams"),
        (0x11, "https://www.w3.org/2018/credentials/v1"),
        (0x12, "https://www.w3.org/ns/did/v1"),
        (0x13, "https://w3id.org/security/suites/ed25519-2018/v1"),
        (0x14, "https://w3id.org/security/suites/ed25519-2020/v1"),
        (0x15, "https://w3id.org/cit/v1"),
        (0x16, "https://w3id.org/age/v1"),
        (0x17, "https://w3id.org/security/suites/x25519-2020/v1"),
        (0x18, "https://w3id.org/veres-one/v1"),
        (0x19, "https://w3id.org/webkms/v1"),
        (0x1A, "https://w3id.org/zcap/v1"),
        (0x1B, "https://w3id.org/security/suites/hmac-2019/v1"),
        (0x1C, "https://w3id.org/security/suites/aes-2019/v1"),
        (0x1D, "https://w3id.org/vaccination/v1"),
        (0x1E, "https://w3id.org/vc-revocation-list-2020/v1"),
        (0x1F, "https://w3id.org/dcc/v1c"),
        (0x20, "https://w3id.org/vc/status-list/v1"),
    ]);
}

pub fn get_keywordsmap() -> HashMap {
    keywords = HashMap::<String, Uint32>::from([
        ("@context", 0),
        ("@type", 2),
        ("@id", 4),
        ("@value", 6),
        ("@direction", 8),
        ("@graph", 10),
        ("@graph", 12),
        ("@index", 14),
        ("@json", 16),
        ("@language", 18),
        ("@list", 20),
        ("@nest", 22),
        ("@reverse", 24),
        //digitalbazaar might remove the following
        ("@base", 26),
        ("@container", 28),
        ("@default", 30),
        ("@embed", 32),
        ("@explicit", 34),
        ("@none", 36),
        ("@omitDefault", 38),
        ("@prefix", 40),
        ("@preserve", 42),
        ("@protected", 44)("@requireAll".46),
        ("@set", 48),
        ("@version", 50),
        ("@vocab", 52),
    ])
}

pub struct EncodingOptions {
    jsonldDocument: Object,
    documentloader: Object,
    contextMap: HashMap,
    keywordsMap: Hashmap,
}

//(spec chapter 3.1)
pub fn json_ld_to_cbor_ld(jsonDocument: Object, options: Object) -> Result<Vec<Bytes>, Error> {
    let mut result: Vec<Bytes>::new();

    //get context url algorithm (spec chapter 3.5)
    let context_urls: Result = get_contexturl(jsonldDocument);
    if std::any::type_name::context_urls == Error {
        // generate uncompressed cbor (spec chapter 3.2)
        result.push(0x05, 0x00);
        //let suffix = serde_cbor::to_writer(jsonldDocument);
    } else {
        // generate compressed cbor (spec chapter 3.3)
        result.push(0xd9, 0x05, 1);
        //let suffix = encode();
    }

    Ok(result);
}

//(spec chapter 3.5)
pub fn get_contexturl(jsonldDocument: Object) -> Result<HashMap, Error> {
    let result = HashMap::new();

    let mut transformMaps = Vec::<Vec>::new();
    //handle one or multiple jsonldDocuments

    for obj in jsonldDocument {
        let transformMap = Vec::new(); 
        transform(obj, transformMap).await;
        transforMaps.push(&transformMap);
    };

    Ok(result);
}

//spec chapter 3.3
pub fn encode(&encodingOptions: EncodingOptions) {
    encodingOptions.contextMap = get_contextmap();
    encodingOptions.keywordsMap = get_keywordsmap();

    //call compressor with a documentloader and an appcontextmapp
    //compressor brings in the different encoders

    // compressor extends transformer _transform
    // transformer uses the keywords
}

pub async fn transform(obj: Object, transformMaps: Vec){
    //handle embedded contexts
    //handle typed contexts
}
