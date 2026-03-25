use soroban_sdk::contracterror;
use soroban_sdk::contractimpl;
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{Address, BytesN, Env, Symbol};
use stellar_analytics::{StellarAnalytics, StellarAnalyticsError, IPFSDataset, CIDImmutability};

#[test]
fn test_cid_immutability() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    // Initialize contract
    StellarAnalytics::initialize(env.clone(), admin.clone());
    
    let dataset_cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string();
    let dataset_hash = BytesN::from_array(&env, &[1; 32]);
    let uploader = Address::generate(&env);
    let decryption_key_hash = BytesN::from_array(&env, &[2; 32]);
    
    // Register dataset
    StellarAnalytics::register_dataset(
        env.clone(),
        dataset_cid.clone(),
        dataset_hash,
        uploader.clone(),
        1024,
        true,
        1,
        Some(decryption_key_hash),
    ).unwrap();
    
    // Request analysis
    let request_id = StellarAnalytics::request_analysis(
        env.clone(),
        uploader.clone(),
        dataset_hash,
        dataset_cid.clone(),
        "test_analysis".to_string(),
        "standard".to_string(),
    ).unwrap();
    
    // Initially, CID should not be immutable
    assert!(!StellarAnalytics::is_cid_immutable(env.clone(), dataset_cid.clone()).unwrap());
    
    // Complete analysis (this should mark CID as immutable)
    StellarAnalytics::complete_analysis(
        env.clone(),
        request_id,
        BytesN::from_array(&env, &[3; 32]),
        100000000000000000, // privacy budget used
        95, // accuracy
        vec![&env, BytesN::from_array(&env, &[4; 32])]
    ).unwrap();
    
    // Now CID should be immutable
    assert!(StellarAnalytics::is_cid_immutable(env.clone(), dataset_cid.clone()).unwrap());
    
    // Get immutability record
    let immutability = StellarAnalytics::get_cid_immutability(env.clone(), dataset_cid.clone()).unwrap();
    assert_eq!(immutability.cid, dataset_cid);
    assert_eq!(immutability.request_id, request_id);
    assert_eq!(immutability.dataset_hash, dataset_hash);
}

#[test]
fn test_enhanced_data_availability() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    StellarAnalytics::initialize(env.clone(), admin.clone());
    
    let dataset_cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string();
    let dataset_hash = BytesN::from_array(&env, &[1; 32]);
    let uploader = Address::generate(&env);
    
    // Register dataset
    StellarAnalytics::register_dataset(
        env.clone(),
        dataset_cid.clone(),
        dataset_hash,
        uploader.clone(),
        1024,
        true,
        1,
        None,
    ).unwrap();
    
    // Initially not pinned
    let result = StellarAnalytics::check_data_availability_enhanced(env.clone(), dataset_cid.clone());
    assert_eq!(result, Err(StellarAnalyticsError::DataNotAvailable));
    
    // Pin the dataset
    StellarAnalytics::pin_dataset(env.clone(), dataset_cid.clone()).unwrap();
    
    // Update availability with Filecoin deal
    StellarAnalytics::update_data_availability(
        env.clone(),
        dataset_cid.clone(),
        true,
        1,
        Some(12345)
    ).unwrap();
    
    // Now should be available
    StellarAnalytics::check_data_availability_enhanced(env.clone(), dataset_cid.clone()).unwrap();
}

#[test]
fn test_hybrid_key_management() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    StellarAnalytics::initialize(env.clone(), admin.clone());
    
    let dataset_cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string();
    let dataset_hash = BytesN::from_array(&env, &[1; 32]);
    let uploader = Address::generate(&env);
    let decryption_key_hash = BytesN::from_array(&env, &[2; 32]);
    
    // Register dataset with key hash
    StellarAnalytics::register_dataset(
        env.clone(),
        dataset_cid.clone(),
        dataset_hash,
        uploader.clone(),
        1024,
        true,
        1,
        Some(decryption_key_hash),
    ).unwrap();
    
    // Store decryption key hash
    StellarAnalytics::store_decryption_key_hash(
        env.clone(),
        dataset_cid.clone(),
        decryption_key_hash,
        "test_key_metadata".to_string(),
    ).unwrap();
    
    // Verify correct key hash
    assert!(StellarAnalytics::verify_decryption_key_hash(
        env.clone(),
        dataset_cid.clone(),
        decryption_key_hash
    ).unwrap());
    
    // Verify incorrect key hash
    let wrong_hash = BytesN::from_array(&env, &[3; 32]);
    assert!(!StellarAnalytics::verify_decryption_key_hash(
        env.clone(),
        dataset_cid.clone(),
        wrong_hash
    ).unwrap());
}

#[test]
fn test_dataset_versioning() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    StellarAnalytics::initialize(env.clone(), admin.clone());
    
    let old_cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string();
    let new_cid = "bafybeihr3h2p6usqjh3z2x7vq7xjxk2j5y6h5q5q5q5q5q5q5q5q5q5q".to_string();
    let old_dataset_hash = BytesN::from_array(&env, &[1; 32]);
    let new_dataset_hash = BytesN::from_array(&env, &[2; 32]);
    let uploader = Address::generate(&env);
    
    // Register original dataset
    StellarAnalytics::register_dataset(
        env.clone(),
        old_cid.clone(),
        old_dataset_hash,
        uploader.clone(),
        1024,
        true,
        1,
        None,
    ).unwrap();
    
    // Create new version
    StellarAnalytics::create_dataset_version(
        env.clone(),
        old_cid.clone(),
        new_cid.clone(),
        new_dataset_hash,
        uploader.clone(),
        2048,
        None,
    ).unwrap();
    
    // Verify new version exists
    let new_dataset = StellarAnalytics::get_dataset(env.clone(), new_cid.clone()).unwrap();
    assert_eq!(new_dataset.version, 2);
    assert_eq!(new_dataset.size_bytes, 2048);
    
    // Verify old version still exists
    let old_dataset = StellarAnalytics::get_dataset(env.clone(), old_cid.clone()).unwrap();
    assert_eq!(old_dataset.version, 1);
    assert_eq!(old_dataset.size_bytes, 1024);
}

#[test]
fn test_auto_pinning_service() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    StellarAnalytics::initialize(env.clone(), admin.clone());
    
    let dataset_cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string();
    let dataset_hash = BytesN::from_array(&env, &[1; 32]);
    let uploader = Address::generate(&env);
    
    // Register dataset
    StellarAnalytics::register_dataset(
        env.clone(),
        dataset_cid.clone(),
        dataset_hash,
        uploader.clone(),
        1024,
        true,
        1,
        None,
    ).unwrap();
    
    // Auto-pin dataset (admin only)
    StellarAnalytics::auto_pin_dataset(env.clone(), dataset_cid.clone()).unwrap();
    
    // Verify dataset is pinned
    let dataset = StellarAnalytics::get_dataset(env.clone(), dataset_cid.clone()).unwrap();
    assert!(dataset.pinned);
}

#[test]
fn test_error_handling() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    StellarAnalytics::initialize(env.clone(), admin.clone());
    
    let non_existent_cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string();
    
    // Test non-existent dataset
    assert_eq!(
        StellarAnalytics::get_dataset(env.clone(), non_existent_cid.clone()),
        Err(StellarAnalyticsError::DatasetNotFound)
    );
    
    // Test non-existent immutability
    assert_eq!(
        StellarAnalytics::get_cid_immutability(env.clone(), non_existent_cid.clone()),
        Err(StellarAnalyticsError::DatasetNotFound)
    );
    
    // Test data availability for non-existent CID
    assert_eq!(
        StellarAnalytics::check_data_availability(env.clone(), non_existent_cid.clone()),
        Err(StellarAnalyticsError::DatasetNotFound)
    );
    
    // Test enhanced data availability for non-existent CID
    assert_eq!(
        StellarAnalytics::check_data_availability_enhanced(env.clone(), non_existent_cid.clone()),
        Err(StellarAnalyticsError::DatasetNotFound)
    );
}

#[test]
fn test_comprehensive_workflow() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let analyst = Address::generate(&env);
    
    StellarAnalytics::initialize(env.clone(), admin.clone());
    
    let dataset_cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string();
    let dataset_hash = BytesN::from_array(&env, &[1; 32]);
    let decryption_key_hash = BytesN::from_array(&env, &[2; 32]);
    
    // 1. Register encrypted dataset
    StellarAnalytics::register_dataset(
        env.clone(),
        dataset_cid.clone(),
        dataset_hash,
        admin.clone(),
        1024,
        true,
        1,
        Some(decryption_key_hash),
    ).unwrap();
    
    // 2. Auto-pin dataset
    StellarAnalytics::auto_pin_dataset(env.clone(), dataset_cid.clone()).unwrap();
    
    // 3. Update availability with Filecoin deal
    StellarAnalytics::update_data_availability(
        env.clone(),
        dataset_cid.clone(),
        true,
        1,
        Some(12345)
    ).unwrap();
    
    // 4. Request analysis
    let request_id = StellarAnalytics::request_analysis(
        env.clone(),
        analyst.clone(),
        dataset_hash,
        dataset_cid.clone(),
        "comprehensive_test".to_string(),
        "high".to_string(),
    ).unwrap();
    
    // 5. Verify CID is not yet immutable
    assert!(!StellarAnalytics::is_cid_immutable(env.clone(), dataset_cid.clone()).unwrap());
    
    // 6. Complete analysis
    StellarAnalytics::complete_analysis(
        env.clone(),
        request_id,
        BytesN::from_array(&env, &[3; 32]),
        100000000000000000,
        98,
        vec![&env, BytesN::from_array(&env, &[4; 32])]
    ).unwrap();
    
    // 7. Verify CID is now immutable
    assert!(StellarAnalytics::is_cid_immutable(env.clone(), dataset_cid.clone()).unwrap());
    
    // 8. Verify decryption key hash
    assert!(StellarAnalytics::verify_decryption_key_hash(
        env.clone(),
        dataset_cid.clone(),
        decryption_key_hash
    ).unwrap());
    
    // 9. Create new version
    let new_cid = "bafybeihr3h2p6usqjh3z2x7vq7xjxk2j5y6h5q5q5q5q5q5q5q5q5q5q5q".to_string();
    let new_dataset_hash = BytesN::from_array(&env, &[5; 32]);
    
    StellarAnalytics::create_dataset_version(
        env.clone(),
        dataset_cid.clone(),
        new_cid.clone(),
        new_dataset_hash,
        admin.clone(),
        2048,
        Some(decryption_key_hash),
    ).unwrap();
    
    // 10. Verify new version is not immutable (no analysis started yet)
    assert!(!StellarAnalytics::is_cid_immutable(env.clone(), new_cid.clone()).unwrap());
    
    // 11. Verify enhanced data availability for both versions
    StellarAnalytics::check_data_availability_enhanced(env.clone(), dataset_cid.clone()).unwrap();
    StellarAnalytics::check_data_availability_enhanced(env.clone(), new_cid.clone()).unwrap();
}
