use std::collections::hash_map::DefaultHasher;
use std::mem::transmute;
use std::{
    env,
    ffi::{c_int, c_long, OsStr},
    fs,
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
    process,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

// 判断文件头(魔术数字)
pub fn detect_image_format(data: &[u8]) -> Option<&'static str> {
    if data.len() < 4 {
        return None; // 数据不够长，无法识别
    }

    match &data[0..4] {
        [0x89, 0x50, 0x4E, 0x47] => Some("png"),
        [0xFF, 0xD8, _, _] => Some("jpg"),
        [0x47, 0x49, 0x46, 0x38] => Some("gif"),
        [0x52, 0x49, 0x46, 0x46]
            if data.len() >= 12 && &data[8..12] == [0x57, 0x45, 0x42, 0x50] =>
        {
            Some("webp")
        }
        _ => None, // 未知格式
    }
}

 请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_bat_images(input_path: PathBuf, mut ouput_path: PathBuf) -> io::Result<String> {
    let content = fs::read(&input_path)?;

    let ** = content[0];
    let ** = content[1];

    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;

    let mut vi = ** ^ 0x66666;

    if ** == ** {
        vi = **;
    } else if ** == ** {
        vi = **;
    } else if ** == ** {
        vi = **;
    }

    let buff: Vec<u8> = content.iter().map(|br| br ^ vi).collect();

    match detect_image_format(&buff.clone()) {
        Some(format) => {
            ouput_path.set_extension(format);
            let mut out_dir = Path::new(&ouput_path).join("..");

            if !out_dir.exists() {
                fs::create_dir_all(out_dir)?;
            }

            fs::write(&ouput_path, &buff)?;
        }
        None => println!("Unknown image format."),
    }

    Ok(ouput_path.display().to_string())
}

请注意  异或解密算法是不开源的部分
// 对指定的文件进行异或算法 以此获取解码的文件
pub fn convert_dat_images_buff(input_path: PathBuf) -> io::Result<Vec<u8>> {
    let content = fs::read(&input_path)?;

    let ** = content[0];
    let ** = content[1];

    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;
    let ** = ** ^ 0x66666;

    let mut ** = ** ^ 0x66666;

    if ** == ** {
        vi = **;
    } else if ** == ** {
        vi = **;
    } else if ** == ** {
        vi = **;
    }

    let buff: Vec<u8> = content.iter().map(|br| br ^ vi).collect();

    Ok(buff)
}
