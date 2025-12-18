use crate::error::Result;
use crate::primitive::Primitive;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Manifest entry for a generated asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    /// Relative path from project root
    pub path: String,
    /// SHA-256 hash of file contents
    pub sha256: String,
    /// Asset type (swatch, tech, progress)
    #[serde(rename = "type")]
    pub asset_type: String,
    /// Primitive parameters that generated this asset
    pub primitive: PrimitiveInfo,
    /// File size in bytes
    pub size_bytes: usize,
}

/// Serializable primitive information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum PrimitiveInfo {
    Swatch {
        color: String,
        style: String,
    },
    Tech {
        name: String,
        bg_color: String,
        logo_color: String,
        style: String,
    },
    Progress {
        percent: u8,
        width: u32,
        height: u32,
    },
    Donut {
        percent: u8,
        size: u32,
        thickness: u32,
    },
    Gauge {
        percent: u8,
        size: u32,
        thickness: u32,
    },
    Sparkline {
        point_count: usize,
        width: u32,
        height: u32,
        chart_type: String,
    },
    Rating {
        value: f32,
        max: u32,
        icon: String,
    },
    Waveform {
        point_count: usize,
        width: u32,
        height: u32,
    },
}

impl From<&Primitive> for PrimitiveInfo {
    fn from(p: &Primitive) -> Self {
        match p {
            Primitive::Swatch { color, style, .. } => PrimitiveInfo::Swatch {
                color: color.clone(),
                style: style.clone(),
            },
            Primitive::Tech {
                name,
                bg_color,
                logo_color,
                style,
                ..
            } => PrimitiveInfo::Tech {
                name: name.clone(),
                bg_color: bg_color.clone(),
                logo_color: logo_color.clone(),
                style: style.clone(),
            },
            Primitive::Progress {
                percent,
                width,
                height,
                ..
            } => PrimitiveInfo::Progress {
                percent: *percent,
                width: *width,
                height: *height,
            },
            Primitive::Donut {
                percent,
                size,
                thickness,
                ..
            } => PrimitiveInfo::Donut {
                percent: *percent,
                size: *size,
                thickness: *thickness,
            },
            Primitive::Gauge {
                percent,
                size,
                thickness,
                ..
            } => PrimitiveInfo::Gauge {
                percent: *percent,
                size: *size,
                thickness: *thickness,
            },
            Primitive::Sparkline {
                values,
                width,
                height,
                chart_type,
                ..
            } => PrimitiveInfo::Sparkline {
                point_count: values.len(),
                width: *width,
                height: *height,
                chart_type: chart_type.clone(),
            },
            Primitive::Rating {
                value, max, icon, ..
            } => PrimitiveInfo::Rating {
                value: *value,
                max: *max,
                icon: icon.clone(),
            },
            Primitive::Waveform {
                values,
                width,
                height,
                ..
            } => PrimitiveInfo::Waveform {
                point_count: values.len(),
                width: *width,
                height: *height,
            },
        }
    }
}

/// Asset manifest for tracking generated files
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetManifest {
    /// Manifest version
    pub version: String,
    /// Timestamp of manifest creation
    pub created_at: String,
    /// Backend used for generation
    pub backend: String,
    /// Assets directory (relative path)
    pub assets_dir: String,
    /// Total number of assets
    pub total_assets: usize,
    /// Asset entries
    pub assets: Vec<AssetEntry>,
}

impl AssetManifest {
    /// Create a new manifest
    pub fn new(backend: &str, assets_dir: &str) -> Self {
        Self {
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            backend: backend.to_string(),
            assets_dir: assets_dir.to_string(),
            total_assets: 0,
            assets: Vec::new(),
        }
    }

    /// Add an asset entry to the manifest
    pub fn add_asset(
        &mut self,
        path: String,
        bytes: &[u8],
        primitive: &Primitive,
        asset_type: String,
    ) {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash = format!("{:x}", hasher.finalize());

        self.assets.push(AssetEntry {
            path,
            sha256: hash,
            asset_type,
            primitive: PrimitiveInfo::from(primitive),
            size_bytes: bytes.len(),
        });

        self.total_assets = self.assets.len();
    }

    /// Write manifest to file
    pub fn write(&self, manifest_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(manifest_path, json)?;
        Ok(())
    }

    /// Load manifest from file
    pub fn load(manifest_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(manifest_path)?;
        let manifest: AssetManifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    /// Get all asset paths from manifest
    pub fn asset_paths(&self) -> Vec<&str> {
        self.assets.iter().map(|a| a.path.as_str()).collect()
    }

    /// Verify that all manifest assets exist on disk with correct hashes
    pub fn verify(&self, base_dir: &Path) -> Vec<VerificationResult> {
        let mut results = Vec::new();

        for entry in &self.assets {
            let full_path = base_dir.join(&entry.path);

            let result = if !full_path.exists() {
                VerificationResult::Missing {
                    path: entry.path.clone(),
                }
            } else {
                match fs::read(&full_path) {
                    Ok(bytes) => {
                        use sha2::{Digest, Sha256};
                        let mut hasher = Sha256::new();
                        hasher.update(&bytes);
                        let actual_hash = format!("{:x}", hasher.finalize());

                        if actual_hash == entry.sha256 {
                            VerificationResult::Valid {
                                path: entry.path.clone(),
                            }
                        } else {
                            VerificationResult::HashMismatch {
                                path: entry.path.clone(),
                                expected: entry.sha256.clone(),
                                actual: actual_hash,
                            }
                        }
                    }
                    Err(e) => VerificationResult::ReadError {
                        path: entry.path.clone(),
                        error: e.to_string(),
                    },
                }
            };

            results.push(result);
        }

        results
    }
}

/// Result of verifying a single asset
#[derive(Debug)]
pub enum VerificationResult {
    Valid {
        path: String,
    },
    Missing {
        path: String,
    },
    HashMismatch {
        path: String,
        expected: String,
        actual: String,
    },
    ReadError {
        path: String,
        error: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Primitive;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_create_manifest() {
        let manifest = AssetManifest::new("svg", "assets/mdfx");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.backend, "svg");
        assert_eq!(manifest.assets_dir, "assets/mdfx");
        assert_eq!(manifest.total_assets, 0);
    }

    #[test]
    fn test_add_asset() {
        let mut manifest = AssetManifest::new("svg", "assets/mdfx");

        let primitive = Primitive::simple_swatch("F41C80", "flat-square");

        let svg_bytes = b"<svg>test</svg>";
        manifest.add_asset(
            "assets/mdfx/swatch_abc123.svg".to_string(),
            svg_bytes,
            &primitive,
            "swatch".to_string(),
        );

        assert_eq!(manifest.total_assets, 1);
        assert_eq!(manifest.assets[0].path, "assets/mdfx/swatch_abc123.svg");
        assert_eq!(manifest.assets[0].asset_type, "swatch");
        assert_eq!(manifest.assets[0].size_bytes, svg_bytes.len());
        assert!(!manifest.assets[0].sha256.is_empty());
    }

    #[test]
    fn test_primitive_info_conversion() {
        let primitive = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "292A2D".to_string(),
            logo_color: "FFFFFF".to_string(),
            style: "flat-square".to_string(),
            label: None,
            border_color: None,
            border_width: None,
            rx: None,
            text_color: None,
            font: None,
            source: None,
        };

        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Tech { name, .. } => assert_eq!(name, "rust"),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_asset_paths() {
        let mut manifest = AssetManifest::new("svg", "assets");
        let primitive = Primitive::simple_swatch("FF0000", "flat");

        manifest.add_asset(
            "path1.svg".to_string(),
            b"svg1",
            &primitive,
            "swatch".to_string(),
        );
        manifest.add_asset(
            "path2.svg".to_string(),
            b"svg2",
            &primitive,
            "swatch".to_string(),
        );

        let paths = manifest.asset_paths();
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&"path1.svg"));
        assert!(paths.contains(&"path2.svg"));
    }

    #[test]
    fn test_write_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("manifest.json");

        let mut manifest = AssetManifest::new("svg", "assets");
        let primitive = Primitive::simple_swatch("FF0000", "flat");
        manifest.add_asset(
            "test.svg".to_string(),
            b"<svg/>",
            &primitive,
            "swatch".to_string(),
        );

        // Write manifest
        manifest.write(&manifest_path).unwrap();
        assert!(manifest_path.exists());

        // Load manifest
        let loaded = AssetManifest::load(&manifest_path).unwrap();
        assert_eq!(loaded.version, "1.0.0");
        assert_eq!(loaded.backend, "svg");
        assert_eq!(loaded.total_assets, 1);
        assert_eq!(loaded.assets[0].path, "test.svg");
    }

    #[test]
    fn test_verify_valid_asset() {
        let temp_dir = TempDir::new().unwrap();
        let asset_path = temp_dir.path().join("test.svg");
        let svg_content = b"<svg>valid</svg>";

        // Write asset file
        let mut file = fs::File::create(&asset_path).unwrap();
        file.write_all(svg_content).unwrap();

        // Create manifest with matching hash
        let mut manifest = AssetManifest::new("svg", ".");
        let primitive = Primitive::simple_swatch("FF0000", "flat");
        manifest.add_asset(
            "test.svg".to_string(),
            svg_content,
            &primitive,
            "swatch".to_string(),
        );

        // Verify
        let results = manifest.verify(temp_dir.path());
        assert_eq!(results.len(), 1);
        match &results[0] {
            VerificationResult::Valid { path } => assert_eq!(path, "test.svg"),
            _ => panic!("Expected Valid result"),
        }
    }

    #[test]
    fn test_verify_missing_asset() {
        let temp_dir = TempDir::new().unwrap();

        let mut manifest = AssetManifest::new("svg", ".");
        let primitive = Primitive::simple_swatch("FF0000", "flat");
        manifest.add_asset(
            "missing.svg".to_string(),
            b"<svg/>",
            &primitive,
            "swatch".to_string(),
        );

        let results = manifest.verify(temp_dir.path());
        assert_eq!(results.len(), 1);
        match &results[0] {
            VerificationResult::Missing { path } => assert_eq!(path, "missing.svg"),
            _ => panic!("Expected Missing result"),
        }
    }

    #[test]
    fn test_verify_hash_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let asset_path = temp_dir.path().join("test.svg");

        // Write asset file with different content
        let mut file = fs::File::create(&asset_path).unwrap();
        file.write_all(b"<svg>modified</svg>").unwrap();

        // Create manifest with original content hash
        let mut manifest = AssetManifest::new("svg", ".");
        let primitive = Primitive::simple_swatch("FF0000", "flat");
        manifest.add_asset(
            "test.svg".to_string(),
            b"<svg>original</svg>",
            &primitive,
            "swatch".to_string(),
        );

        let results = manifest.verify(temp_dir.path());
        assert_eq!(results.len(), 1);
        match &results[0] {
            VerificationResult::HashMismatch { path, .. } => assert_eq!(path, "test.svg"),
            _ => panic!("Expected HashMismatch result"),
        }
    }
}
