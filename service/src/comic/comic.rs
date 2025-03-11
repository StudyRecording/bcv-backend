use std::io::Read;
use utils::err::ResultErr;
use utils::zip_read::get_archive;

pub async fn get_zip_picture(index: usize) -> Result<Vec<u8>, ResultErr> {
    let mut archive = get_archive("C:/Users/28763/Pictures/Wallper.zip")?;
    let mut file = archive.by_index(index).map_err(|_| ResultErr::BizErr {msg: "获取压缩图片失败".into()})?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).map_err(|_| ResultErr::BizErr {msg: "获取图片bytes失败".into()})?;
    Ok(buf)
}