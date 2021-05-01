use serde_derive::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Compose {
    version: String,
    services: HashMap<String, Service>
}

#[derive(Deserialize, Debug)]
struct Service {
    image: String,
    build: Option<String>,
}



fn main() -> anyhow::Result<()> {
    let f = std::fs::File::open("docker-compose.yaml").
        or_else(|_| std::fs::File::open("docker-compose.yml"))?;
    let c: Compose = serde_yaml::from_reader(std::io::BufReader::new(f))?;

    for (_, service) in c.services.iter() {
        if let Some(builddir) = service.build.as_ref() {
            let mut kaniko = std::process::Command::new("/kaniko/executor")
                .arg("--context").arg(builddir)
                .arg("--dockerfile").arg(format!("{}/Dockerfile", builddir))
                .arg("--destination").arg(&service.image)
                .arg("--insecure")
                .spawn()?;
            let status = kaniko.wait()?;
            if status.success() {
                println!("build and pushed {}", service.image);
            }
            else{
                anyhow::bail!("kaniko failed for {}", service.image);
            }
        }
    }

    println!("{:?}", c);
    Ok(())
}
