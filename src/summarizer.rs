use crate::github::{PrInfo, FileStatus};
use anyhow::Result;
use itertools::Itertools;

pub struct Summary {
    pub description: String,
    pub affected_files: String,
}

pub fn generate_summary(pr_info: &PrInfo) -> Result<Summary> {
    // Generate bullet points based on the PR description and changed files
    let mut description_points = Vec::new();
    
    // If the PR has a title that's meaningful, add it
    if !pr_info.title.is_empty() {
        description_points.push(format!("- {}", pr_info.title));
    }
    
    // Try to extract key points from the PR description
    if !pr_info.description.is_empty() {
        let extracted_points = extract_key_points(&pr_info.description);
        description_points.extend(extracted_points);
    }
    
    // If we still don't have enough points, infer from file changes
    if description_points.is_empty() || description_points.len() < 2 {
        let inferred_points = infer_changes_from_files(&pr_info.changed_files);
        for point in inferred_points {
            if !description_points.contains(&point) {
                description_points.push(point);
            }
        }
    }
    
    // Format affected files section
    let affected_files = format_affected_files(&pr_info.changed_files);
    
    // Join description points with newlines
    let description = description_points.join("\n");
    
    Ok(Summary {
        description,
        affected_files,
    })
}

fn extract_key_points(description: &str) -> Vec<String> {
    let mut points = Vec::new();
    
    // Split by lines and look for patterns that suggest bullet points
    for line in description.lines() {
        let trimmed = line.trim();
        
        // Check if it's already a bullet point or numbered item
        if trimmed.starts_with('-') || trimmed.starts_with('*') || 
           (trimmed.len() > 2 && trimmed.chars().next().unwrap().is_numeric() && trimmed.chars().nth(1) == Some('.')) {
            // Clean up the bullet point
            let content = trimmed
                .trim_start_matches('-')
                .trim_start_matches('*')
                .trim_start_matches(|c: char| c.is_numeric() || c == '.')
                .trim();

            if !content.is_empty() {
                points.push(format!("- {}", content));
            }
        }
    }
    // Limit to 5 points max
    points.truncate(5);
    points
}

fn infer_changes_from_files(files: &[crate::github::ChangedFile]) -> Vec<String> {
    let mut points = Vec::new();
    
    // Group files by directory to understand component changes
    let files_by_dir = files.iter()
        .map(|file| {
            let parts: Vec<&str> = file.filename.split('/').collect();
            let dir = if parts.len() > 1 { parts[0] } else { "" };
            (dir, file)
        })
        .into_group_map();
    
    // Check for specific patterns
    let has_tests = files.iter().any(|f| f.filename.contains("test") || f.filename.contains("spec"));
    if has_tests {
        points.push("- Added or updated tests".to_string());
    }
    
    let has_docs = files.iter().any(|f| 
        f.filename.ends_with(".md") || 
        f.filename.ends_with(".rst") || 
        f.filename.contains("doc") || 
        f.filename.eq("README.md")
    );
    if has_docs {
        points.push("- Updated documentation".to_string());
    }
    
    // Look at file extensions to identify the primary changes
    let mut _file_extensions = files.iter()
        .filter_map(|f| {
            let parts: Vec<&str> = f.filename.split('.').collect();
            if parts.len() > 1 { Some(parts.last().unwrap().to_string()) } else { None }
        })
        .collect::<Vec<String>>();
    
    // Generate points based on file groups
    for (dir, dir_files) in files_by_dir {
        if dir.is_empty() || dir_files.len() < 2 {
            continue;
        }
        
        let added = dir_files.iter().filter(|f| matches!(f.status, FileStatus::Added)).count();
        let modified = dir_files.iter().filter(|f| matches!(f.status, FileStatus::Modified)).count();
        let removed = dir_files.iter().filter(|f| matches!(f.status, FileStatus::Removed)).count();
        
        if added > 0 && added == dir_files.len() {
            points.push(format!("- Added new {} component/module", dir));
        } else if removed > 0 && removed == dir_files.len() {
            points.push(format!("- Removed {} component/module", dir));
        } else if modified > 0 {
            points.push(format!("- Updated {} component/module", dir));
        }
    }
    
    // Ensure we have at least one point
    if points.is_empty() && !files.is_empty() {
        points.push(format!("- Modified {} files", files.len()));
    }
    
    points
}

fn format_affected_files(files: &[crate::github::ChangedFile]) -> String {
    let mut result = String::new();
    
    for file in files {
        let status = match file.status {
            FileStatus::Added => "[+]",
            FileStatus::Modified => "[M]",
            FileStatus::Removed => "[-]",
            FileStatus::Renamed => "[R]",
        };
        
        result.push_str(&format!("- {} {}\n", status, file.filename));
    }
    
    result
}