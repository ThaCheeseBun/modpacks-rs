mod types;

use std::fs::{self, File};
use std::path::Path;
use std::{env, io};

use reqwest::blocking::Client;
use sha1::{Digest, Sha1};
use sha2::Sha512;

use types::*;

fn download_from_manifest(manifest: modpacksch::VersionManifest) {
    // create folder
    let mut target_dir = env::current_dir().unwrap();
    target_dir.push(format!("{}_{}", manifest.parent, manifest.id));
    fs::create_dir_all(&target_dir).unwrap();

    // download all files
    let download_client = Client::new();
    for file in manifest.files {
        // get info from manifest
        let target = Path::join(&target_dir, file.path);
        let target_file = Path::join(&target, &file.name);

        println!("Downloading \"{}\"...", file.name);

        // get curseforge url if from there
        let target_url = match file.curseforge {
            Some(cf) => format!(
                "https://www.curseforge.com/api/v1/mods/{}/files/{}/download",
                cf.project, cf.file
            ),
            _ => file.url,
        };

        // download and compare sha1 hash
        let res = download_client.get(target_url).send().unwrap();
        let bytes = res.bytes().unwrap();
        let sha = base16ct::lower::encode_string(&Sha1::digest(&bytes));
        if sha != file.sha1 {
            eprintln!("sha1 hash invalid, {:?}", file.name);
            break;
        }

        // create output directory
        fs::create_dir_all(target).unwrap();

        // write to file
        let mut ofile = File::create(target_file).unwrap();
        io::Write::write_all(&mut ofile, &bytes).unwrap();
    }
}

fn convert_from_manifest_to_mrpack(manifest: modpacksch::VersionManifest) {
    let mut output_files: Vec<modrinth::FormatFile> = vec![];

    let download_client = Client::new();
    for file in manifest.files {
        // get curseforge url if from there
        let target_url = match file.curseforge {
            Some(cf) => format!(
                "https://www.curseforge.com/api/v1/mods/{}/files/{}/download",
                cf.project, cf.file
            ),
            _ => file.url,
        };

        println!("Downloading \"{}\"...", file.name);

        // download and compare sha1 hash
        let res = download_client.get(&target_url).send().unwrap();
        let bytes = res.bytes().unwrap();
        let sha1 = base16ct::lower::encode_string(&Sha1::digest(&bytes));
        if sha1 != file.sha1 {
            eprintln!("sha1 hash invalid, {:?}", file.name);
            break;
        }

        // generate sha512 hash
        let sha512 = base16ct::lower::encode_string(&Sha512::digest(&bytes));

        output_files.push(modrinth::FormatFile {
            path: format!("{}{}", file.path, file.name),
            hashes: modrinth::FormatFileHashes { sha1, sha512 },
            env: Some(modrinth::FormatFileEnv {
                client: if file.clientonly || !file.serveronly {
                    if file.optional {
                        "optional"
                    } else {
                        "required"
                    }
                } else {
                    "unsupported"
                }
                .to_owned(),
                server: if !file.clientonly || file.serveronly {
                    if file.optional {
                        "optional"
                    } else {
                        "required"
                    }
                } else {
                    "unsupported"
                }
                .to_owned(),
            }),
            downloads: vec![target_url],
            file_size: file.size,
        });
    }

    let mut output_deps = modrinth::FormatDeps {
        minecraft: None,
        forge: None,
        neoforge: None,
        fabric_loader: None,
        quilt_loader: None,
    };
    for target in manifest.targets {
        match target.name.as_str() {
            "minecraft" => output_deps.minecraft = Some(target.version),
            "forge" => output_deps.forge = Some(target.version),
            "neoforge" => output_deps.neoforge = Some(target.version),
            "fabric" => output_deps.fabric_loader = Some(target.version),
            _ => {}
        };
    }

    let thing = modrinth::Format {
        format_version: 1,
        game: "minecraft".to_owned(),
        version_id: manifest.name,
        name: "example_name".to_owned(),
        summary: None,
        files: output_files,
        dependencies: output_deps,
    };

    let json = serde_json::to_string_pretty(&thing).unwrap();
    fs::write("modrinth.index.json", json).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 3 {
        eprintln!("Invalid amount of arguments.");
        return;
    }

    // get manifest from provided link
    let res = reqwest::blocking::get(&args[2]).unwrap();
    let manifest: modpacksch::VersionManifest =
        serde_json::from_slice(&res.bytes().unwrap()).unwrap();

    if args[1] == "download" {
        download_from_manifest(manifest);
    } else if args[1] == "convert" {
        convert_from_manifest_to_mrpack(manifest);
    } else {
        println!("Invalid action \"{}\"", args[1]);
    }
}
