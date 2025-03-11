use crate::err::ResultErr;
use std::fs::File;
use zip::ZipArchive;

/// 获取解压文件信息
pub fn get_archive(path: &str) -> Result<ZipArchive<File>, ResultErr> {
    let zip_file = File::open(path)
        .map_err(|_e| { ResultErr::BizErr { msg: "解压文件打开失败".into() } })?;
    let archive = ZipArchive::new(zip_file)
        .map_err(|_e| ResultErr::BizErr {msg: "解压文件信息获取失败".into()})?;
    
    Ok(archive)
}

/// 读取压缩文件数量
pub fn get_zip_file_num(path: &str) -> Result<usize, ResultErr>{
    
    let archive = get_archive(path)?;
    Ok(archive.len())
}