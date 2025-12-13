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
    /// Asset type (swatch, divider, tech, status)
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
    Divider {
        colors: Vec<String>,
        style: String,
    },
    Tech {
        name: String,
        bg_color: String,
        logo_color: String,
        style: String,
    },
    Status {
        level: String,
        style: String,
    },
}

impl From<&Primitive> for PrimitiveInfo {
    fn from(p: &Primitive) -> Self {
        match p {
            Primitive::Swatch { color, style } => PrimitiveInfo::Swatch {
                color: color.clone(),
                style: style.clone(),
            },
            Primitive::Divider { colors, style } => PrimitiveInfo::Divider {
                colors: colors.clone(),
                style: style.clone(),
            },
            Primitive::Tech {
                name,
                bg_color,
                logo_color,
                style,
            } => PrimitiveInfo::Tech {
                name: name.clone(),
                bg_color: bg_color.clone(),
                logo_color: logo_color.clone(),
                style: style.clone(),
            },
            Primitive::Status { level, style } => PrimitiveInfo::Status {
                level: level.clone(),
                style: style.clone(),
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

        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
        };

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
        };

        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Tech { name, .. } => assert_eq!(name, "rust"),
            _ => panic!("Wrong variant"),
        }
    }
}
