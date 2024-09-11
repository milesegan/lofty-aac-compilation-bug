use std::path::Path;

use lofty::{file::TaggedFileExt, probe::Probe, tag::ItemKey};

fn main() {
    let path = Path::new("sample.m4a");

    let tagged_file = Probe::open(path)
        .expect("no probe")
        .read()
        .expect("no read");

    let tag = tagged_file.primary_tag().expect("no primary tag");
    println!(
        "compilation: {:?}",
        tag.get_string(&ItemKey::FlagCompilation)
    );
}
