use std::io::Result;

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();

    prost_build.include_file(format!(
        "{}/{}",
        std::env::var("OUT_DIR").unwrap(),
        "_includes.rs"
    ));

    // prost_build.type_attribute(".", "");

    prost_build.compile_protos(
        &[
            "protos/plugin/metadata.proto",
            "protos/cron/cron_job.proto",
            "protos/message/message.proto",
            "protos/message/reaction.proto",
            "protos/database/prepared_statement.proto",
            "protos/database/results.proto",
            "protos/hank.proto",
        ],
        &["protos"],
    )?;

    Ok(())
}
