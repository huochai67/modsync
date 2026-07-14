use crate::error::Error;

use std::{
    fs::{read_dir, File},
    io::{BufReader, Read},
    path::Path,
};

#[allow(non_snake_case)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModMetaMods {
    pub modId: String,
    pub version: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModMeta {
    pub mods: Vec<ModMetaMods>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MSMOD {
    pub md5: String,
    pub path: String,
    pub size: usize,
    pub url: Option<String>,
    pub modid: Option<String>,
    pub version: Option<String>,
}

impl MSMOD {
    pub fn new(
        md5: String,
        path: String,
        size: usize,
        url: Option<String>,
        modid: Option<String>,
        version: Option<String>,
    ) -> MSMOD {
        MSMOD {
            md5,
            path,
            size,
            url,
            modid,
            version,
        }
    }

    pub fn clone(&self) -> MSMOD {
        MSMOD {
            md5: self.md5.clone(),
            path: self.path.clone(),
            size: self.size,
            url: self.url.clone(),
            modid: self.modid.clone(),
            version: self.version.clone(),
        }
    }

    pub fn from_jsonfile(filepath: &str) -> Result<Vec<MSMOD>, Error> {
        let mut file = File::open(filepath)?;
        let mut str: String = "".to_string();
        file.read_to_string(&mut str)?;
        Ok(serde_json::from_str::<Vec<MSMOD>>(str.as_str())?)
    }

    pub fn from_file(
        filepath: &Path,
        parentpath: &str,
        serverurl: Option<&str>,
    ) -> Result<MSMOD, Error> {
        let mut file = BufReader::new(File::open(filepath)?);
        let mut buffer = [0_u8; 64 * 1024];
        let mut size = 0_usize;
        let mut digest = md5::Context::new();
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            digest.consume(&buffer[..bytes_read]);
            size += bytes_read;
        }

        let strfilepath = filepath.to_string_lossy().to_string();
        let strpath = if parentpath.is_empty() {
            filepath
                .file_name()
                .ok_or_else(|| Error::Validation("file has no name".to_string()))?
                .to_string_lossy()
                .replace('\\', "/")
        } else {
            filepath
                .strip_prefix(parentpath)
                .map_err(|_| {
                    Error::Validation(format!(
                        "{} is not under {}",
                        filepath.display(),
                        parentpath
                    ))
                })?
                .to_string_lossy()
                .replace('\\', "/")
        };

        let digest = digest.compute();
        let url = match serverurl {
            Some(su) => Some(format!("{}{}", su, strpath)),
            None => None,
        };

        let mut modid = Option::None;
        let mut version = Option::None;
        if strfilepath.ends_with(".jar") {
            if let Ok(file) = std::fs::File::open(filepath) {
                let reader = std::io::BufReader::new(file);
                if let Ok(mut archive) = zip::ZipArchive::new(reader) {
                    for i in 0..archive.len() {
                        if let Ok(mut zipfile) = archive.by_index(i) {
                            if zipfile.name() == "META-INF/MANIFEST.MF" {
                                let mut contents = String::new();
                                if let Ok(_rsize) = zipfile.read_to_string(&mut contents) {
                                    for line in contents.lines() {
                                        let data: Vec<&str> = line.split(": ").collect();
                                        if data[0] == "Specification-Title" {
                                            modid = Some(data[1].to_string());
                                        }
                                        if data[0] == "Implementation-Version" {
                                            version = Some(data[1].to_string());
                                        }
                                    }
                                }
                            }
                            if zipfile.name() == "META-INF/mods.toml" {
                                let mut contents = String::new();
                                if let Ok(_rsize) = zipfile.read_to_string(&mut contents) {
                                    if let Ok(modmeta) =
                                        toml::from_str::<ModMeta>(contents.as_str())
                                    {
                                        if modmeta.mods.len() > 0 {
                                            modid = Option::Some(modmeta.mods[0].modId.clone());
                                            if modmeta.mods[0].version != "${file.jarVersion}" {
                                                version =
                                                    Option::Some(modmeta.mods[0].version.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        Ok(MSMOD::new(
            format!("{:X}", digest),
            strpath,
            size,
            url,
            modid,
            version,
        ))
    }

    pub fn from_directory_impl(
        filedir: &str,
        rootdir: &str,
        serverurl: Option<&str>,
    ) -> Result<Vec<MSMOD>, Error> {
        let mut ret: Vec<MSMOD> = vec![];
        let entrys = read_dir(filedir)?;
        for entry_ in entrys {
            let entry = entry_?;
            let entrytype = entry.file_type()?;
            if entrytype.is_dir() {
                match Self::from_directory_impl(&entry.path().to_string_lossy(), rootdir, serverurl)
                {
                    Ok(mut ret2) => ret.append(&mut ret2),
                    Err(err) => return Err(err),
                };
            }
            if entrytype.is_file() {
                let path = entry.path();
                ret.push(MSMOD::from_file(path.as_path(), rootdir, serverurl)?);
            }
        }
        Ok(ret)
    }
    pub fn from_directory(filedir: &str, serverurl: Option<&str>) -> Result<Vec<MSMOD>, Error> {
        Self::from_directory_impl(filedir, filedir, serverurl)
    }
}
