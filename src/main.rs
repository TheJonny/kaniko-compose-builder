use serde::Deserialize;

use indexmap::IndexMap;

use anyhow::Context;

#[derive(Deserialize, Debug)]
struct Compose {
    version: String,
    services: IndexMap<String, Service>
}

#[derive(Deserialize, Debug)]
struct Service {
    image: String,
    build: Option<serde_yaml::Value>,
}

#[derive(Deserialize, Debug)]
struct Build{
    context: String,
    dockerfile: String,
}

fn main() -> anyhow::Result<()> {
    let f = std::fs::File::open("docker-compose.yaml").
        or_else(|_| std::fs::File::open("docker-compose.yml"))?;
    let c: Compose = serde_yaml::from_reader(std::io::BufReader::new(f))?;

    for (_, service) in c.services.iter() {
        if let Some(build_val) = service.build.as_ref() {
            let build: Build = match build_val {
                serde_yaml::Value::String(s) => Build { context: s.to_owned(), dockerfile: "Dockerfile".to_owned() },
                serde_yaml::Value::Mapping(_) => serde_yaml::from_value::<Build>(build_val.clone()).context("could not parse build key")?,
                _ => anyhow::bail!("build must be map or string"),
            };
            let mut kaniko = std::process::Command::new("/kaniko/executor")
                .arg("--context").arg(&build.context)
                .arg("--dockerfile").arg(format!("{}/{}", &build.context, &build.dockerfile))
                .arg("--destination").arg(&service.image)
                .args(&std::env::args().skip(1).collect::<Vec<String>>())
                .spawn()?;
            let status = kaniko.wait()?;
            if status.success() {
                println!("built and pushed {}", service.image);
            }
            else{
                anyhow::bail!("kaniko failed for {}", service.image);
            }
        }
    
    }

    println!("{:?}", c);
    Ok(())
}
