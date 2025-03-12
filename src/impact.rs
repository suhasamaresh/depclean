use crate::analysis::DuplicateSet;

//Estimate of impact of removing duplicate dependencies
pub struct ImpactEstimate {
    pub name: String, 
    pub potential_savings: usize,// for example, in kilobytes (KB)
    pub cur_versions: Vec<String>,
}

pub fn estimate_impact(duplicates : &[DuplicateSet]) -> Vec<ImpactEstimate> {
    duplicates.iter().map(|dup| {
        // A mock estimation: e.g., assume each duplicate version contributes 10 KB overhead.
        let package_size = 10;
        ImpactEstimate {
            name: dup.name.clone(),
            potential_savings: (dup.versions.len() - 1) * package_size,
            cur_versions: dup.versions.clone(),
        }
    }).collect()
}