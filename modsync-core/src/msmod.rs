use crate::error::Error;

use std::{
    fs::{read_dir, File},
    io::Read,
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

    pub fn from_file(filepath: &Path, parentpath: &str, serverurl: Option<&str>) -> MSMOD {
        let mut file = File::open(filepath).unwrap();
        let mut buffer = Vec::new();
        let size = file.read_to_end(&mut buffer).unwrap();

        let strfilepath = filepath.as_os_str().to_str().unwrap().to_string();
        let strpath = &strfilepath[parentpath.len() + 1..].replace('\\', "/");

        let digest = md5::compute(buffer);
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

        MSMOD::new(
            format!("{:X}", digest),
            strpath.to_string(),
            size,
            url,
            modid,
            version,
        )
    }

    pub fn from_directory_impl(
        filedir: &str,
        rootdir: &str,
        serverurl: Option<&str>,
    ) -> Result<Vec<Option<MSMOD>>, Error> {
        let mut ret: Vec<Option<MSMOD>> = vec![];
        let entrys = read_dir(filedir)?;
        for entry_ in entrys {
            let entry = entry_?;
            let entrytype = entry.file_type()?;
            if entrytype.is_dir() {
                match Self::from_directory_impl(
                    entry.path().to_str().unwrap(),
                    rootdir,
                    serverurl,
                ) {
                    Ok(mut ret2) => ret.append(&mut ret2),
                    Err(err) => return Err(err),
                };
            }
            if entrytype.is_file() {
                let path = entry.path();
                ret.push(Some(MSMOD::from_file(path.as_path(), rootdir, serverurl)));
            }
        }
        Ok(ret)
    }
    pub fn from_directory(
        filedir: &str,
        serverurl: Option<&str>,
    ) -> Result<Vec<Option<MSMOD>>, Error> {
        Self::from_directory_impl(filedir, filedir, serverurl)
    }
}
