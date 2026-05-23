use nis2_rules::engine::{self, ANNEX_I_SECTORS, ANNEX_II_SECTORS};
use nis2_rules::schema::{CompanyProfile, EntityCategory};

struct TestCase {
    description: String,
    profile: CompanyProfile,
    expected_applicable: bool,
    expected_category: EntityCategory,
}

#[test]
fn test_108_case_regression_matrix() {
    let mut cases = Vec::new();

    let all_sectors = [
        // Annex I
        "energy", "transport", "banking", "financial_market_infrastructure",
        "health", "drinking_water", "waste_water", "digital_infrastructure",
        "ict_service_management_b2b", "public_administration", "space",
        // Annex II
        "postal_courier", "waste_management", "chemicals", "food",
        "manufacturing", "digital_providers", "research",
        // Out of Scope
        "retail",
    ];

    for sector in all_sectors {
        // Size configuration 1: Micro (OutOfScope unless special sector)
        let mut p1 = CompanyProfile {
            name: format!("Micro {}", sector),
            sector: sector.to_string(),
            sub_sector: None,
            employees: 5,
            annual_revenue_eur_m: 0.5,
            balance_sheet_eur_m: 0.5,
            services: vec![],
            member_states: vec!["IT".to_string()],
        };
        // Special services setups
        if sector == "digital_infrastructure" {
            p1.services = vec!["Cloud hosting".to_string()]; // not exempt
        }
        let exp_cat_1 = match sector {
            "ict_service_management_b2b" | "public_administration" => EntityCategory::Essential,
            "digital_infrastructure" => EntityCategory::OutOfScope, // because it has non-exempt cloud services
            _ => EntityCategory::OutOfScope,
        };
        cases.push(TestCase {
            description: format!("Micro {}", sector),
            profile: p1,
            expected_applicable: exp_cat_1 != EntityCategory::OutOfScope,
            expected_category: exp_cat_1,
        });

        // Size configuration 2: Small (OutOfScope unless special sector)
        let mut p2 = CompanyProfile {
            name: format!("Small {}", sector),
            sector: sector.to_string(),
            sub_sector: None,
            employees: 49,
            annual_revenue_eur_m: 9.5,
            balance_sheet_eur_m: 9.5,
            services: vec![],
            member_states: vec!["IT".to_string()],
        };
        if sector == "digital_infrastructure" {
            p2.services = vec!["dns".to_string()]; // always exempt
        }
        let exp_cat_2 = match sector {
            "ict_service_management_b2b" | "public_administration" => EntityCategory::Essential,
            "digital_infrastructure" => EntityCategory::Essential, // because of DNS service
            _ => EntityCategory::OutOfScope,
        };
        cases.push(TestCase {
            description: format!("Small {}", sector),
            profile: p2,
            expected_applicable: exp_cat_2 != EntityCategory::OutOfScope,
            expected_category: exp_cat_2,
        });

        // Size configuration 3: Medium (Meets headcount threshold)
        let p3 = CompanyProfile {
            name: format!("Med Headcount {}", sector),
            sector: sector.to_string(),
            sub_sector: None,
            employees: 50,
            annual_revenue_eur_m: 5.0,
            balance_sheet_eur_m: 5.0,
            services: vec![],
            member_states: vec!["IT".to_string()],
        };
        let is_i = ANNEX_I_SECTORS.contains(&sector);
        let is_ii = ANNEX_II_SECTORS.contains(&sector);
        let exp_cat_3 = if is_i {
            EntityCategory::Essential
        } else if is_ii {
            EntityCategory::Important
        } else {
            EntityCategory::OutOfScope
        };
        cases.push(TestCase {
            description: format!("Med Headcount {}", sector),
            profile: p3,
            expected_applicable: exp_cat_3 != EntityCategory::OutOfScope,
            expected_category: exp_cat_3,
        });

        // Size configuration 4: Medium (Meets financial thresholds - both >= 10M)
        let p4 = CompanyProfile {
            name: format!("Med Financial Both {}", sector),
            sector: sector.to_string(),
            sub_sector: None,
            employees: 10,
            annual_revenue_eur_m: 12.0,
            balance_sheet_eur_m: 11.0,
            services: vec![],
            member_states: vec!["IT".to_string()],
        };
        let exp_cat_4 = if is_i {
            EntityCategory::Essential
        } else if is_ii {
            EntityCategory::Important
        } else {
            EntityCategory::OutOfScope
        };
        cases.push(TestCase {
            description: format!("Med Financial Both {}", sector),
            profile: p4,
            expected_applicable: exp_cat_4 != EntityCategory::OutOfScope,
            expected_category: exp_cat_4,
        });

        // Size configuration 5: Medium (Fails financial thresholds - revenue >= 10M but balance < 10M)
        let mut p5 = CompanyProfile {
            name: format!("Med Financial Fails {}", sector),
            sector: sector.to_string(),
            sub_sector: None,
            employees: 10,
            annual_revenue_eur_m: 12.0,
            balance_sheet_eur_m: 8.0,
            services: vec![],
            member_states: vec!["IT".to_string()],
        };
        if sector == "digital_infrastructure" {
            p5.services = vec!["TLD Registry".to_string()]; // exempt
        }
        let exp_cat_5 = match sector {
            "ict_service_management_b2b" | "public_administration" => EntityCategory::Essential,
            "digital_infrastructure" => EntityCategory::Essential, // always exempt TLD
            _ => EntityCategory::OutOfScope, // small enterprise because balance sheet is <10M and employees <50
        };
        cases.push(TestCase {
            description: format!("Med Financial Fails {}", sector),
            profile: p5,
            expected_applicable: exp_cat_5 != EntityCategory::OutOfScope,
            expected_category: exp_cat_5,
        });

        // Size configuration 6: Large (Meets all thresholds)
        let p6 = CompanyProfile {
            name: format!("Large {}", sector),
            sector: sector.to_string(),
            sub_sector: None,
            employees: 300,
            annual_revenue_eur_m: 150.0,
            balance_sheet_eur_m: 130.0,
            services: vec![],
            member_states: vec!["IT".to_string()],
        };
        let exp_cat_6 = if is_i {
            EntityCategory::Essential
        } else if is_ii {
            EntityCategory::Important
        } else {
            EntityCategory::OutOfScope
        };
        cases.push(TestCase {
            description: format!("Large {}", sector),
            profile: p6,
            expected_applicable: exp_cat_6 != EntityCategory::OutOfScope,
            expected_category: exp_cat_6,
        });
    }

    assert_eq!(cases.len(), 114, "Should run exactly 114 regression tests (19 sectors * 6 configs)");

    let mut failed = 0;
    for c in cases {
        let result = engine::evaluate(&c.profile);
        if result.applicable != c.expected_applicable || result.category != c.expected_category {
            println!(
                "FAIL: {} | Sector: {} | Emp: {} | Rev: {} | Bal: {}\nExpected applicable: {}, cat: {:?}\nGot applicable: {}, cat: {:?}",
                c.description,
                c.profile.sector,
                c.profile.employees,
                c.profile.annual_revenue_eur_m,
                c.profile.balance_sheet_eur_m,
                c.expected_applicable,
                c.expected_category,
                result.applicable,
                result.category
            );
            failed += 1;
        }
    }

    assert_eq!(failed, 0, "{} / 114 regression cases failed evaluation logic!", failed);
}
