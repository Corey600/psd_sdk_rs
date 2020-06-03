extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let vendor = manifest_dir.join("vendor");

    let mut cc = cc::Build::new();
    cc.cpp(true);
    cc.include(&vendor);
    cc.define("NDEBUG", Some("1"));
    cc.define("_THREAD_SAFE", Some("1"));
    cc.flag("-fvisibility=hidden");
    cc.flag("-std=c++11");

    let files = [
        "src/Psd/PsdExport.cpp",
        "src/Psd/PsdPch.cpp",
        "src/Psd/PsdDecompressRle.cpp",
        "src/Psd/PsdInterleave.cpp",
        "src/Psd/PsdLayerCanvasCopy.cpp",
        "src/Psd/PsdAllocator.cpp",
        "src/Psd/PsdFile.cpp",
        "src/Psd/PsdMallocAllocator.cpp",
        "src/Psd/PsdNativeFile_Mac.mm",
        "src/Psd/PsdParseColorModeDataSection.cpp",
        "src/Psd/PsdParseDocument.cpp",
        "src/Psd/PsdParseImageDataSection.cpp",
        "src/Psd/PsdParseImageResourcesSection.cpp",
        "src/Psd/PsdParseLayerMaskSection.cpp",
        "src/Psd/PsdBlendMode.cpp",
        "src/Psd/PsdColorMode.cpp",
        "src/Psd/PsdFixedSizeString.cpp",
        "src/Psd/PsdSyncFileReader.cpp",
        "src/Psd/Psdminiz.c",
        "src/Psd/PsdSyncFileWriter.cpp",
    ];
    for f in files.iter() {
        cc.file(vendor.join(f));
    }

    cc.compile("psd_sdk");
}
