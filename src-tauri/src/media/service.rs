use crate::domain::{EntryId, MediaSource, Photo};
use std::path::{Path, PathBuf};

pub struct MediaService {
    base_dir: PathBuf,
}

impl MediaService {
    pub fn new(base_dir: PathBuf) -> Self {
        let originals = base_dir.join("originals");
        let thumbnails = base_dir.join("thumbnails");
        let _ = std::fs::create_dir_all(&originals);
        let _ = std::fs::create_dir_all(&thumbnails);
        Self { base_dir }
    }

    pub fn save_original(&self, filename: &str, data: &[u8]) -> Result<PathBuf, String> {
        let path = self.base_dir.join("originals").join(filename);
        std::fs::write(&path, data).map_err(|e| e.to_string())?;
        Ok(path)
    }

    pub fn generate_thumbnail(&self, filename: &str, max_dim: u32) -> Result<PathBuf, String> {
        let orig_path = self.base_dir.join("originals").join(filename);
        let thumb_path = self.base_dir.join("thumbnails").join(filename);

        if let Ok(img) = image::open(&orig_path) {
            let resized = img.thumbnail(max_dim, max_dim);
            let _ = resized.save(&thumb_path);
        } else {
            // Fallback copy if non-image byte stream
            let _ = std::fs::copy(&orig_path, &thumb_path);
        }

        Ok(thumb_path)
    }

    pub fn register_photo(&self, entry_id: EntryId, filename: &str, source: MediaSource) -> Photo {
        let orig = self.base_dir.join("originals").join(filename);
        let thumb = self.base_dir.join("thumbnails").join(filename);
        Photo::with_source(
            entry_id,
            orig.to_string_lossy().to_string(),
            thumb.to_string_lossy().to_string(),
            source,
        )
    }
}
