use std::{
    fs::{read_dir, File},
    io::Read,
    path::Path,
};

use toml::{value::Array, Table};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MSMOD {
    pub md5: String,
    pub path: String,
    pub size: usize,
    pub url: Option<String>,
    pub modid: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModMetaMods {
    pub modId: String,
    pub version: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModMeta {
    pub mods: Vec<ModMetaMods>,
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
        let strpath = &strfilepath[parentpath.len() + 1..];

        let digest = md5::compute(buffer);
        let url = match serverurl {
            Some(su) => Some(format!("{}data/mods/{}", su, strpath)),
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
                            if zipfile.name() == "META-INF/mods.toml" {
                                let mut contents = String::new();
                                if let Ok(_rsize) = zipfile.read_to_string(&mut contents) {
                                    if let Ok(modmeta) =
                                        toml::from_str::<ModMeta>(contents.as_str())
                                    {
                                        if modmeta.mods.len() > 0 {
                                            modid = Option::Some(modmeta.mods[0].modId.clone());
                                            version = Option::Some(modmeta.mods[0].version.clone());
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
    ) -> Result<Vec<Option<MSMOD>>, Box<dyn std::error::Error + Send>> {
        let mut ret: Vec<Option<MSMOD>> = vec![];
        match read_dir(filedir) {
            Ok(entrys) => {
                for entry_ in entrys {
                    match entry_ {
                        Ok(entry) => match entry.file_type() {
                            Ok(entrytype) => {
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
                                    ret.push(Some(MSMOD::from_file(
                                        path.as_path(),
                                        rootdir,
                                        serverurl,
                                    )));
                                }
                            }
                            Err(err) => return Err(Box::new(err)),
                        },
                        Err(err) => return Err(Box::new(err)),
                    }
                }
            }
            Err(err) => return Err(Box::new(err)),
        }
        Ok(ret)
    }
    pub fn from_directory(
        filedir: &str,
        serverurl: Option<&str>,
    ) -> Result<Vec<Option<MSMOD>>, Box<dyn std::error::Error + Send>> {
        Self::from_directory_impl(filedir, filedir, serverurl)
    }
}
