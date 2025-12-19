//! Asset manifest system for tracking generated files
//!
//! Features:
//! - SHA-256 content verification
//! - Atomic manifest writes (prevents corruption)
//! - Incremental manifest updates (merge capability)
//! - Provenance tracking (source files, version, timestamp)
//! - Content-addressed filenames (stable across Rust versions)

use crate::error::Result;
use crate::primitive::Primitive;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Current manifest schema version
pub const MANIFEST_VERSION: &str = "1.1.0";

/// Manifest entry for a generated asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    /// Relative path from project root
    pub path: String,
    /// SHA-256 hash of file contents (for verification)
    pub sha256: String,
    /// Asset type (swatch, tech, progress)
    #[serde(rename = "type")]
    pub asset_type: String,
    /// Primitive parameters that generated this asset
    pub primitive: PrimitiveInfo,
    /// File size in bytes
    pub size_bytes: usize,

    // Provenance tracking fields (v1.1.0)
    /// Source template files that reference this asset
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_files: Vec<String>,
    /// Timestamp when this asset was generated (RFC3339)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
    /// mdfx version that generated this asset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator_version: Option<String>,
}

impl AssetEntry {
    /// Create a new asset entry with provenance tracking
    pub fn new(
        path: String,
        bytes: &[u8],
        primitive: &Primitive,
        asset_type: String,
        source_file: Option<String>,
    ) -> Self {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash = format!("{:x}", hasher.finalize());

        let source_files = source_file.into_iter().collect();

        Self {
            path,
            sha256: hash,
            asset_type,
            primitive: PrimitiveInfo::from(primitive),
            size_bytes: bytes.len(),
            source_files,
            generated_at: Some(chrono::Utc::now().to_rfc3339()),
            generator_version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }
    }
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
            Primitive::Tech(cfg) => PrimitiveInfo::Tech {
                name: cfg.name.clone(),
                bg_color: cfg.bg_color.clone(),
                logo_color: cfg.logo_color.clone(),
                style: cfg.style.clone(),
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
    /// Manifest schema version (for migrations)
    pub version: String,
    /// Timestamp of manifest creation/update
    pub created_at: String,
    /// Backend used for generation
    pub backend: String,
    /// Assets directory (relative path)
    pub assets_dir: String,
    /// Total number of assets
    pub total_assets: usize,
    /// Total size of all assets in bytes
    #[serde(default)]
    pub total_size_bytes: usize,
    /// mdfx version that generated this manifest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator_version: Option<String>,
    /// Asset entries
    pub assets: Vec<AssetEntry>,
}

impl AssetManifest {
    /// Create a new manifest
    pub fn new(backend: &str, assets_dir: &str) -> Self {
        Self {
            version: MANIFEST_VERSION.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            backend: backend.to_string(),
            assets_dir: assets_dir.to_string(),
            total_assets: 0,
            total_size_bytes: 0,
            generator_version: Some(env!("CARGO_PKG_VERSION").to_string()),
            assets: Vec::new(),
        }
    }

    /// Add an asset entry to the manifest (legacy API for backwards compatibility)
    pub fn add_asset(
        &mut self,
        path: String,
        bytes: &[u8],
        primitive: &Primitive,
        asset_type: String,
    ) {
        self.add_asset_with_source(path, bytes, primitive, asset_type, None);
    }

    /// Add an asset entry with source file tracking
    pub fn add_asset_with_source(
        &mut self,
        path: String,
        bytes: &[u8],
        primitive: &Primitive,
        asset_type: String,
        source_file: Option<String>,
    ) {
        let entry = AssetEntry::new(path, bytes, primitive, asset_type, source_file);
        self.total_size_bytes += entry.size_bytes;
        self.assets.push(entry);
        self.total_assets = self.assets.len();
    }

    /// Add a pre-built asset entry
    pub fn add_entry(&mut self, entry: AssetEntry) {
        self.total_size_bytes += entry.size_bytes;
        self.assets.push(entry);
        self.total_assets = self.assets.len();
    }

    /// Merge new assets into this manifest (incremental update)
    ///
    /// This keeps existing assets that are still valid and adds/updates new ones.
    /// Assets not in `new_paths` are removed (they're stale).
    ///
    /// # Arguments
    /// * `new_assets` - New asset entries to merge in
    /// * `new_paths` - Set of paths that should exist after merge (for cleanup)
    pub fn merge(&mut self, new_assets: Vec<AssetEntry>, new_paths: Option<HashSet<String>>) {
        // Build a map of new assets by path
        let new_by_path: std::collections::HashMap<_, _> = new_assets
            .into_iter()
            .map(|a| (a.path.clone(), a))
            .collect();

        // If new_paths provided, filter existing assets to only keep those still needed
        if let Some(ref keep_paths) = new_paths {
            self.assets.retain(|a| keep_paths.contains(&a.path));
        }

        // Update existing or add new
        for (path, new_entry) in new_by_path {
            if let Some(existing) = self.assets.iter_mut().find(|a| a.path == path) {
                // Update existing entry, preserving source_files if new has none
                let mut merged_sources = existing.source_files.clone();
                for src in &new_entry.source_files {
                    if !merged_sources.contains(src) {
                        merged_sources.push(src.clone());
                    }
                }
                *existing = AssetEntry {
                    source_files: merged_sources,
                    ..new_entry
                };
            } else {
                // Add new entry
                self.assets.push(new_entry);
            }
        }

        // Recalculate totals
        self.total_assets = self.assets.len();
        self.total_size_bytes = self.assets.iter().map(|a| a.size_bytes).sum();
        self.created_at = chrono::Utc::now().to_rfc3339();
    }

    /// Write manifest to file (standard write)
    pub fn write(&self, manifest_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(manifest_path, json)?;
        Ok(())
    }

    /// Write manifest atomically (prevents corruption on crash/interrupt)
    ///
    /// This writes to a temporary file first, then atomically renames it.
    /// If the process crashes during write, the original manifest is preserved.
    pub fn write_atomic(&self, manifest_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;

        // Write to temp file in same directory (important for atomic rename)
        let temp_path = manifest_path.with_extension("json.tmp");
        fs::write(&temp_path, &json)?;

        // Atomic rename (on most filesystems)
        fs::rename(&temp_path, manifest_path)?;

        Ok(())
    }

    /// Load manifest from file with version migration support
    pub fn load(manifest_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(manifest_path)?;

        // Parse as generic JSON first to check version
        let raw: serde_json::Value = serde_json::from_str(&content)?;
        let version = raw
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0");

        // Migrate if needed
        let manifest: AssetManifest = match version {
            "1.0.0" => {
                // v1.0.0 → v1.1.0: Add new fields with defaults
                let mut m: AssetManifest = serde_json::from_value(raw)?;
                m.version = MANIFEST_VERSION.to_string();
                // New fields get defaults via serde
                m
            }
            "1.1.0" => serde_json::from_value(raw)?,
            _ => {
                // Unknown version, try to parse anyway
                serde_json::from_value(raw)?
            }
        };

        Ok(manifest)
    }

    /// Get all asset paths from manifest
    pub fn asset_paths(&self) -> Vec<&str> {
        self.assets.iter().map(|a| a.path.as_str()).collect()
    }

    /// Get asset by path
    pub fn get_asset(&self, path: &str) -> Option<&AssetEntry> {
        self.assets.iter().find(|a| a.path == path)
    }

    /// Check if an asset with given SHA-256 hash exists
    pub fn has_content_hash(&self, sha256: &str) -> bool {
        self.assets.iter().any(|a| a.sha256 == sha256)
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

    /// Get summary statistics
    pub fn stats(&self) -> ManifestStats {
        let mut by_type: std::collections::HashMap<String, TypeStats> =
            std::collections::HashMap::new();

        for asset in &self.assets {
            let stats = by_type
                .entry(asset.asset_type.clone())
                .or_insert(TypeStats {
                    count: 0,
                    total_bytes: 0,
                });
            stats.count += 1;
            stats.total_bytes += asset.size_bytes;
        }

        let largest_asset = self
            .assets
            .iter()
            .max_by_key(|a| a.size_bytes)
            .map(|a| a.path.clone());

        ManifestStats {
            total_assets: self.total_assets,
            total_size_bytes: self.total_size_bytes,
            by_type,
            largest_asset,
        }
    }
}

/// Statistics about asset types
#[derive(Debug, Clone)]
pub struct TypeStats {
    pub count: usize,
    pub total_bytes: usize,
}

/// Summary statistics for manifest
#[derive(Debug)]
pub struct ManifestStats {
    pub total_assets: usize,
    pub total_size_bytes: usize,
    pub by_type: std::collections::HashMap<String, TypeStats>,
    pub largest_asset: Option<String>,
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

/// Generate content-addressed filename from SVG bytes
///
/// Uses first 16 characters of SHA-256 hash for stable, unique filenames.
/// This is deterministic across Rust versions (unlike DefaultHasher).
pub fn content_addressed_filename(bytes: &[u8], type_prefix: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = format!("{:x}", hasher.finalize());

    // Use first 16 chars of SHA-256 (64 bits of entropy, sufficient for dedup)
    format!("{}_{}.svg", type_prefix, &hash[..16])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Primitive;
    use rstest::rstest;
    use std::io::Write;
    use tempfile::TempDir;

    // ========================================================================
    // Manifest Creation (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("svg", "assets/mdfx")]
    #[case("shields", "assets")]
    #[case("png", "images/generated")]
    fn test_create_manifest(#[case] backend: &str, #[case] assets_dir: &str) {
        let manifest = AssetManifest::new(backend, assets_dir);
        assert_eq!(manifest.version, MANIFEST_VERSION);
        assert_eq!(manifest.backend, backend);
        assert_eq!(manifest.assets_dir, assets_dir);
        assert_eq!(manifest.total_assets, 0);
        assert!(manifest.generator_version.is_some());
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
    fn test_add_asset_with_source() {
        let mut manifest = AssetManifest::new("svg", "assets");
        let primitive = Primitive::simple_swatch("FF0000", "flat");

        manifest.add_asset_with_source(
            "test.svg".to_string(),
            b"<svg/>",
            &primitive,
            "swatch".to_string(),
            Some("README.md".to_string()),
        );

        assert_eq!(manifest.assets[0].source_files, vec!["README.md"]);
        assert!(manifest.assets[0].generated_at.is_some());
        assert!(manifest.assets[0].generator_version.is_some());
    }

    #[test]
    fn test_primitive_info_conversion() {
        use crate::primitive::TechConfig;
        let primitive = Primitive::Tech(TechConfig {
            name: "rust".to_string(),
            bg_color: "292A2D".to_string(),
            logo_color: "FFFFFF".to_string(),
            ..Default::default()
        });

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
        assert_eq!(loaded.version, MANIFEST_VERSION);
        assert_eq!(loaded.backend, "svg");
        assert_eq!(loaded.total_assets, 1);
        assert_eq!(loaded.assets[0].path, "test.svg");
    }

    #[test]
    fn test_write_atomic() {
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

        // Atomic write
        manifest.write_atomic(&manifest_path).unwrap();
        assert!(manifest_path.exists());

        // Verify temp file is gone
        let temp_path = manifest_path.with_extension("json.tmp");
        assert!(!temp_path.exists());

        // Load and verify
        let loaded = AssetManifest::load(&manifest_path).unwrap();
        assert_eq!(loaded.total_assets, 1);
    }

    #[test]
    fn test_merge_manifests() {
        let mut manifest = AssetManifest::new("svg", "assets");
        let primitive = Primitive::simple_swatch("FF0000", "flat");

        // Add initial assets
        manifest.add_asset_with_source(
            "old.svg".to_string(),
            b"<svg>old</svg>",
            &primitive,
            "swatch".to_string(),
            Some("file1.md".to_string()),
        );

        // Create new assets
        let new_entry = AssetEntry::new(
            "new.svg".to_string(),
            b"<svg>new</svg>",
            &primitive,
            "swatch".to_string(),
            Some("file2.md".to_string()),
        );

        // Merge with new paths (old.svg not in new_paths, so it's removed)
        let new_paths: HashSet<String> = ["new.svg".to_string()].into_iter().collect();
        manifest.merge(vec![new_entry], Some(new_paths));

        assert_eq!(manifest.total_assets, 1);
        assert_eq!(manifest.assets[0].path, "new.svg");
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

    // ========================================================================
    // Content-Addressed Filename (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(b"<svg>content1</svg>", "swatch")]
    #[case(b"<svg>content2</svg>", "tech")]
    #[case(b"<svg>progress</svg>", "progress")]
    fn test_content_addressed_filename(#[case] content: &[u8], #[case] prefix: &str) {
        let name = content_addressed_filename(content, prefix);

        // Correct format
        assert!(name.starts_with(&format!("{}_", prefix)));
        assert!(name.ends_with(".svg"));
        assert_eq!(name.len(), prefix.len() + 1 + 16 + 4); // prefix + "_" + hash + ".svg"

        // Same content = same filename (deterministic)
        assert_eq!(name, content_addressed_filename(content, prefix));
    }

    #[test]
    fn test_content_addressed_different_content() {
        let svg1 = b"<svg>content1</svg>";
        let svg2 = b"<svg>content2</svg>";

        let name1 = content_addressed_filename(svg1, "swatch");
        let name2 = content_addressed_filename(svg2, "swatch");

        // Different content = different filename
        assert_ne!(name1, name2);
    }

    #[test]
    fn test_stats() {
        let mut manifest = AssetManifest::new("svg", "assets");
        let primitive = Primitive::simple_swatch("FF0000", "flat");

        manifest.add_asset(
            "swatch1.svg".to_string(),
            b"<svg>1</svg>",
            &primitive,
            "swatch".to_string(),
        );
        manifest.add_asset(
            "swatch2.svg".to_string(),
            b"<svg>22</svg>",
            &primitive,
            "swatch".to_string(),
        );

        let stats = manifest.stats();
        assert_eq!(stats.total_assets, 2);
        assert!(stats.total_size_bytes > 0);
        assert!(stats.by_type.contains_key("swatch"));
        assert_eq!(stats.by_type["swatch"].count, 2);
    }

    // ========================================================================
    // PrimitiveInfo Conversion Tests (Parameterized)
    // ========================================================================

    #[test]
    fn test_primitive_info_from_progress() {
        let primitive = Primitive::simple_progress(75, "#E0E0E0", "#4CAF50");
        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Progress {
                percent,
                width,
                height,
            } => {
                assert_eq!(percent, 75);
                assert_eq!(width, 100); // default
                assert_eq!(height, 10); // default
            }
            _ => panic!("Expected Progress variant"),
        }
    }

    #[test]
    fn test_primitive_info_from_donut() {
        let primitive = Primitive::simple_donut(50, "#E0E0E0", "#2196F3");
        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Donut {
                percent,
                size,
                thickness,
            } => {
                assert_eq!(percent, 50);
                assert_eq!(size, 40); // default
                assert_eq!(thickness, 4); // default
            }
            _ => panic!("Expected Donut variant"),
        }
    }

    #[test]
    fn test_primitive_info_from_gauge() {
        let primitive = Primitive::Gauge {
            percent: 80,
            size: 100,
            thickness: 10,
            track_color: "#E0E0E0".to_string(),
            fill_color: "#FF5722".to_string(),
            show_label: true,
            label_color: Some("#000000".to_string()),
            thumb_size: None,
            thumb_color: None,
        };
        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Gauge {
                percent,
                size,
                thickness,
            } => {
                assert_eq!(percent, 80);
                assert_eq!(size, 100);
                assert_eq!(thickness, 10);
            }
            _ => panic!("Expected Gauge variant"),
        }
    }

    #[test]
    fn test_primitive_info_from_sparkline() {
        let primitive = Primitive::Sparkline {
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            width: 120,
            height: 30,
            chart_type: "line".to_string(),
            fill_color: "#9C27B0".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 3,
        };
        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Sparkline {
                point_count,
                width,
                height,
                chart_type,
            } => {
                assert_eq!(point_count, 5);
                assert_eq!(width, 120);
                assert_eq!(height, 30);
                assert_eq!(chart_type, "line");
            }
            _ => panic!("Expected Sparkline variant"),
        }
    }

    #[test]
    fn test_primitive_info_from_rating() {
        let primitive = Primitive::Rating {
            value: 4.0,
            max: 5,
            icon: "star".to_string(),
            fill_color: "#FFD700".to_string(),
            empty_color: "#E0E0E0".to_string(),
            size: 20,
            spacing: 2,
        };
        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Rating { value, max, icon } => {
                assert!((value - 4.0).abs() < f32::EPSILON);
                assert_eq!(max, 5);
                assert_eq!(icon, "star");
            }
            _ => panic!("Expected Rating variant"),
        }
    }

    #[test]
    fn test_primitive_info_from_waveform() {
        let primitive = Primitive::Waveform {
            values: vec![0.5, 0.8, 0.3, 0.9, 0.6],
            width: 100,
            height: 40,
            positive_color: "#00BCD4".to_string(),
            negative_color: "#FF5722".to_string(),
            bar_width: 3,
            spacing: 1,
            track_color: None,
            show_center_line: false,
            center_line_color: None,
        };
        let info = PrimitiveInfo::from(&primitive);
        match info {
            PrimitiveInfo::Waveform {
                point_count,
                width,
                height,
            } => {
                assert_eq!(point_count, 5);
                assert_eq!(width, 100);
                assert_eq!(height, 40);
            }
            _ => panic!("Expected Waveform variant"),
        }
    }
}
