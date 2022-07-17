#![no_main]
use kdl::KdlDocument;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let doc: Result<KdlDocument, _> = data.parse();
    if let Ok(doc) = doc {
        assert_eq!(data, doc.to_string());
    }
});
