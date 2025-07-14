use crate::prelude::*;

pub fn get_detailed_file_info(
    path: PathBuf,
    file_name: &mut String,
    total_blocks: Option<&mut u64>,
    max_len: &mut usize,
    flags: &Flag,
) -> Result<Vec<String>, String> {
    let metadata = path
        .symlink_metadata()
        .map_err(|e| format!("cannot access metadata of '{}': {}", path.display(), e))?;

    let permission = get_permissions(&metadata, path.to_path_buf());

    let size = if metadata.file_type().is_char_device() || metadata.file_type().is_block_device() {
        let (major, minor) = get_major_minor(&metadata);
        let mut res = String::new();

        res.push_str(&major.to_string());
        res.push_str(", ");
        res.push_str(&minor.to_string());
        res
    } else {
        metadata.len().to_string()
    };

    if size.len() > *max_len {
        *max_len = size.len();
    }

    format_path(path.to_path_buf(), file_name, flags)?;

    let (user_owner, group_owner) = get_owners_info(&metadata)
        .map_err(|e| format!("cannot access '{}': {}", path.display(), e))?;

    let n_link = metadata.nlink().to_string();

    let modified_at = get_modified_at(&metadata);

    if let Some(blocks) = total_blocks {
        *blocks += metadata.blocks() / 2;
    }

    Ok(vec![
        permission,
        n_link,
        user_owner,
        group_owner,
        size,
        modified_at,
        (*file_name).to_string(),
    ])
}

fn get_modified_at(metadata: &Metadata) -> String {
    match metadata.modified() {
        Ok(modified_at) => {
            let datetime_utc: DateTime<Utc> = modified_at.into();

            let datetime = Casablanca.from_utc_datetime(&datetime_utc.naive_utc());

            let now = Casablanca.from_utc_datetime(&Utc::now().naive_utc());
            let six_months_ago = now - Duration::days(30 * 6);

            if datetime > six_months_ago && datetime <= now {
                datetime.format("%b %e %H:%M").to_string()
            } else {
                datetime.format("%b %e  %Y").to_string()
            }
        }
        Err(_) => "<invalid time>".to_string(),
    }
}

fn get_owners_info(metadata: &Metadata) -> Result<(String, String), String> {
    let uid = metadata.uid();
    let gid = metadata.gid();

    unsafe {
        let passwd = libc::getpwuid(uid);

        let username = if !passwd.is_null() {
            CStr::from_ptr((*passwd).pw_name)
                .to_str()
                .map_err(|_| format!("Invalid UTF-8 in group name for UID({uid})"))?
                .to_string()
        } else {
            uid.to_string()
        };

        let group = libc::getgrgid(gid);
        let groupname = if !group.is_null() {
            CStr::from_ptr((*group).gr_name)
                .to_str()
                .map_err(|_| format!("Invalid UTF-8 in group name for GID({gid})"))?
                .to_string()
        } else {
            gid.to_string()
        };

        Ok((username, groupname))
    }
}
