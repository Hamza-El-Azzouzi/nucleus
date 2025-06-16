use std::ffi::CStr;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

pub fn clean_string(s: String) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}

pub fn get_modified_at(metadata: &Metadata) -> String {
    match metadata.modified() {
        Ok(modified_at) => {
            let datetime: chrono::DateTime<chrono::Local> = modified_at.into();
            datetime.format("%b %e %H:%M").to_string()
        }
        Err(_) => "<invalid time>".to_string(),
    }
}

pub fn get_file_owner_and_group(metadata: &Metadata) -> Result<(String, String), String> {
    let uid = metadata.uid();
    let gid = metadata.gid();

    unsafe {
        let passwd = libc::getpwuid(uid);
        if passwd.is_null() {
            return Err(format!("Failed to get user name for UID({})", uid));
        }
        let username = CStr::from_ptr((*passwd).pw_name)
            .to_str()
            .map_err(|_| format!("Invalid UTF-8 in user name for UID({})", uid))?
            .to_string();

        let group = libc::getgrgid(gid);
        if group.is_null() {
            return Err(format!("Failed to get group name for GID({})", gid));
        }
        let groupname = CStr::from_ptr((*group).gr_name)
            .to_str()
            .map_err(|_| format!("Invalid UTF-8 in group name for GID({})", gid))?
            .to_string();

        Ok((username, groupname))
    }
}

pub fn get_permission_string(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();

    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.file_type().is_symlink() {
        'l'
    } else {
        '-'
    };

    let mut result = String::new();
    result.push(file_type);

    let bits = [
        (mode >> 6) & 0b111, // user
        (mode >> 3) & 0b111, // group
        (mode >> 0) & 0b111, // others
    ];

    for &part in &bits {
        result.push(if part & 0b100 != 0 { 'r' } else { '-' });
        result.push(if part & 0b010 != 0 { 'w' } else { '-' });
        result.push(if part & 0b001 != 0 { 'x' } else { '-' });
    }

    result
}
