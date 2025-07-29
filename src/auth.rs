//! Post-Quantum Cryptography authentication module

use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Post-Quantum Cryptography imports (optional)
#[cfg(feature = "post-quantum")]
use pqcrypto_dilithium::dilithium2::{
    keypair as dilithium_keypair, 
    sign, 
    open, 
    PublicKey as DilithiumPublicKey, 
    SecretKey as DilithiumSecretKey,
    SignedMessage as DilithiumSignedMessage,
};
#[cfg(feature = "post-quantum")]
use pqcrypto_traits::sign::{
    PublicKey as PQPublicKey, 
    SecretKey as PQSecretKey,
    SignedMessage as PQSignedMessage,
};

use crate::SharedError;

/// Post-Quantum authentication structure
#[derive(Debug, Clone)]
pub struct PQAuthentication {
    pub algorithm: String,
    pub version: String,
}

/// Post-Quantum key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PQKeyPair {
    pub public_key: String,  // Base64 encoded
    pub private_key: String, // Base64 encoded
    pub algorithm: String,
    pub created_at: DateTime<Utc>,
    pub key_id: Uuid,
}

/// Authentication challenge structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthChallenge {
    pub challenge_id: Uuid,
    pub challenge_text: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub email: String,
}

/// Signature verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureVerification {
    pub is_valid: bool,
    pub verified_at: DateTime<Utc>,
    pub public_key_id: Option<Uuid>,
    pub message: String,
}

/// JWT claims for Post-Quantum authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PQClaims {
    pub sub: String,        // Subject (user ID)
    pub email: String,      // User email
    pub role: String,       // User role
    pub exp: i64,          // Expiration timestamp
    pub iat: i64,          // Issued at timestamp
    pub iss: String,       // Issuer
    pub aud: String,       // Audience
    pub key_id: Uuid,      // Public key ID used for verification
    pub algorithm: String, // PQ algorithm used
}

impl PQAuthentication {
    /// Create new Post-Quantum authentication instance
    pub fn new() -> Self {
        Self {
            algorithm: "CRYSTALS-Dilithium5".to_string(),
            version: "1.0".to_string(),
        }
    }

    /// Generate a new Post-Quantum key pair
    pub fn generate_keypair(&self) -> Result<PQKeyPair, SharedError> {
        let (public_key, private_key) = dilithium_keypair();
        
        let public_key_b64 = general_purpose::STANDARD.encode(public_key.as_bytes());
        let private_key_b64 = general_purpose::STANDARD.encode(private_key.as_bytes());
        
        Ok(PQKeyPair {
            public_key: public_key_b64,
            private_key: private_key_b64,
            algorithm: self.algorithm.clone(),
            created_at: Utc::now(),
            key_id: Uuid::new_v4(),
        })
    }

    /// Sign a message using Post-Quantum cryptography
    pub fn sign_message(&self, message: &str, private_key_b64: &str) -> Result<String, SharedError> {
        // Decode private key
        let private_key_bytes = general_purpose::STANDARD
            .decode(private_key_b64)
            .map_err(|e| SharedError::CryptographicError(format!("Invalid private key: {}", e)))?;
        
        if private_key_bytes.len() != 4896 {
            return Err(SharedError::CryptographicError(
                format!("Invalid private key length: {} (expected 4896)", private_key_bytes.len())
            ));
        }

        let mut sk_array = [0u8; 4896];
        sk_array.copy_from_slice(&private_key_bytes);
        let secret_key = DilithiumSecretKey::from_bytes(&sk_array)
            .map_err(|e| SharedError::CryptographicError(format!("Failed to construct private key: {}", e)))?;

        // Sign the message
        let signed_message = sign(message.as_bytes(), &secret_key);
        let signature_b64 = general_purpose::STANDARD.encode(PQSignedMessage::as_bytes(&signed_message));
        
        Ok(signature_b64)
    }

    /// Verify a signature using Post-Quantum cryptography
    pub fn verify_signature(&self, message: &str, signature_b64: &str, public_key_b64: &str) -> Result<SignatureVerification, SharedError> {
        // Decode signature and public key
        let signature_bytes = general_purpose::STANDARD
            .decode(signature_b64)
            .map_err(|e| SharedError::CryptographicError(format!("Invalid signature: {}", e)))?;
        
        let public_key_bytes = general_purpose::STANDARD
            .decode(public_key_b64)
            .map_err(|e| SharedError::CryptographicError(format!("Invalid public key: {}", e)))?;
        
        if public_key_bytes.len() != 2592 {
            return Ok(SignatureVerification {
                is_valid: false,
                verified_at: Utc::now(),
                public_key_id: None,
                message: "Invalid public key length".to_string(),
            });
        }

        let mut pk_array = [0u8; 2592];
        pk_array.copy_from_slice(&public_key_bytes);
        let public_key = DilithiumPublicKey::from_bytes(&pk_array)
            .map_err(|e| SharedError::CryptographicError(format!("Failed to construct public key: {}", e)))?;

        // Reconstruct signed message
        let signed_message = PQSignedMessage::from_bytes(&signature_bytes)
            .map_err(|e| SharedError::CryptographicError(format!("Invalid signed message: {}", e)))?;

        // Verify signature
        match open(&signed_message, &public_key) {
            Ok(verified_message) => {
                let is_valid = verified_message == message.as_bytes();
                Ok(SignatureVerification {
                    is_valid,
                    verified_at: Utc::now(),
                    public_key_id: None, // Would be set if we had key tracking
                    message: if is_valid { "Signature verified".to_string() } else { "Message mismatch".to_string() },
                })
            },
            Err(_) => Ok(SignatureVerification {
                is_valid: false,
                verified_at: Utc::now(),
                public_key_id: None,
                message: "Signature verification failed".to_string(),
            }),
        }
    }

    /// Create authentication challenge
    pub fn create_challenge(&self, email: &str) -> AuthChallenge {
        let challenge_id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::minutes(5); // 5-minute expiry
        
        AuthChallenge {
            challenge_id,
            challenge_text: format!("MyDR24-Auth-Challenge-{}-{}", email, now.timestamp()),
            created_at: now,
            expires_at,
            email: email.to_string(),
        }
    }

    /// Validate authentication challenge
    pub fn validate_challenge(&self, challenge: &AuthChallenge) -> bool {
        Utc::now() < challenge.expires_at
    }
}

impl Default for PQAuthentication {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let auth = PQAuthentication::new();
        let keypair = auth.generate_keypair().unwrap();
        
        assert_eq!(keypair.algorithm, "CRYSTALS-Dilithium5");
        assert!(!keypair.public_key.is_empty());
        assert!(!keypair.private_key.is_empty());
    }

    #[test]
    fn test_sign_and_verify() {
        let auth = PQAuthentication::new();
        let keypair = auth.generate_keypair().unwrap();
        let message = "Test message for MyDR24";
        
        let signature = auth.sign_message(message, &keypair.private_key).unwrap();
        let verification = auth.verify_signature(message, &signature, &keypair.public_key).unwrap();
        
        assert!(verification.is_valid);
    }

    #[test]
    fn test_challenge_creation() {
        let auth = PQAuthentication::new();
        let challenge = auth.create_challenge("test@mydr24.com");
        
        assert_eq!(challenge.email, "test@mydr24.com");
        assert!(challenge.challenge_text.contains("MyDR24-Auth-Challenge"));
        assert!(auth.validate_challenge(&challenge));
    }
}
