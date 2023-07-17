/*
 * Created Date: 2023-02-10 19:55:13
 * Author: Virgil-N
 * Description:
 * -----
 * Last Modified: 2023-02-10 19:57:24
 * Modified By: Virgil-N (ghost@earth.com)
 * -----
 * Copyright (c) 2019 - 2023 ⚐
 * Virgil-N will save your soul!
 * -----
 */

use std::path::{Path, PathBuf},
/// 递归获取目录下文件路径并返回
pub fn generate_file_path(
    dir: &Path,
    paths: &mut Vec<PathBuf>,
) -> Result<Vec<PathBuf>, std::io::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                generate_file_path(&path, paths)?;
            } else {
                paths.push(path);
            }
        }
    }
    Ok(paths.to_vec())
}
